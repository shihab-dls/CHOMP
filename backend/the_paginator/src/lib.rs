#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use async_trait::async_trait;
use sea_orm::{
    sea_query::{
        Alias, ColumnRef, Expr, IntoIden, Query, SeaRc, SelectStatement, SimpleExpr, UnionType,
        Values, WindowStatement,
    },
    Condition, ConnectionTrait, DbErr, DynIden, EntityTrait, FromQueryResult, Iden, Iterable,
    Order, OrderedStatement, QueryTrait, Value,
};

/// The contents of a cursor indexed page, with indicators for the existance of previous and next pages.
#[derive(Debug, PartialEq, Eq)]
pub struct CursorPage<I> {
    /// The rows found in the page
    pub items: Vec<I>,
    /// True if at least one row exists before this page
    pub has_previous: bool,
    /// True if at least one row exists after this page
    pub has_next: bool,
}

const HAS_PREVIOUS: &str = "has_previous";
const HAS_NEXT: &str = "has_next";
const NEIGHBOURS_PREFIX: &str = "neighbours_";

#[derive(Debug)]
struct Neighbours {
    pub has_previous: bool,
    pub has_next: bool,
}

impl FromQueryResult for Neighbours {
    fn from_query_result(res: &sea_orm::QueryResult, pre: &str) -> Result<Self, DbErr> {
        Ok(Self {
            has_previous: res.try_get(pre, HAS_PREVIOUS)?,
            has_next: res.try_get(pre, HAS_NEXT)?,
        })
    }
}

/// Allows for querying of pages from a selection.
#[async_trait]
pub trait QueryCursorPage {
    /// The type of item returned as page contents.
    type Item;

    /// Get a page of limited size after the cursor.
    async fn page_after<DbConn>(
        cursor: Option<Values>,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        DbConn: ConnectionTrait;
}

#[async_trait]
impl<E> QueryCursorPage for E
where
    E: EntityTrait,
{
    type Item = E::Model;

    async fn page_after<DbConn>(
        from: Option<Values>,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        DbConn: ConnectionTrait,
    {
        let base_table_prefix = "book_";

        let cursor_by = E::PrimaryKey::iter()
            .map(|pk_idx| SeaRc::new(pk_idx) as SeaRc<dyn Iden>)
            .collect::<Vec<_>>();
        let prefixed_cursor_by = cursor_by
            .iter()
            .map(|pk_idx| {
                Alias::new(&format!("{base_table_prefix}{}", pk_idx.to_string())).into_iden()
            })
            .collect::<Vec<_>>();

        let stmt = Query::select()
            .column(ColumnRef::Asterisk)
            .from_subquery(
                Query::select()
                    .column(ColumnRef::Asterisk)
                    .expr_window_as(
                        Expr::cust_with_values("LAG(TRUE, $1, FALSE)", [1_i32]),
                        WindowStatement::new()
                            .apply_order_by(&prefixed_cursor_by, Order::Asc)
                            .to_owned(),
                        Alias::new(&format!("{NEIGHBOURS_PREFIX}{HAS_PREVIOUS}")),
                    )
                    .expr_window_as(
                        Expr::cust_with_values("LEAD(TRUE, $1, FALSE)", [limit as i32]),
                        WindowStatement::new()
                            .apply_order_by(&prefixed_cursor_by, Order::Asc)
                            .to_owned(),
                        Alias::new(&format!("{NEIGHBOURS_PREFIX}{HAS_NEXT}")),
                    )
                    .from_subquery(
                        Query::select()
                            .column(ColumnRef::Asterisk)
                            .from_subquery(
                                E::find()
                                    .into_query()
                                    .apply_prefix(base_table_prefix)
                                    .apply_order_by(&cursor_by, Order::Desc)
                                    .apply_filter(&cursor_by, from.clone(), |c, v| {
                                        Expr::col(SeaRc::clone(c)).lte(v)
                                    })
                                    .limit(1)
                                    .to_owned(),
                                Alias::new("before").into_iden(),
                            )
                            .union(
                                UnionType::All,
                                E::find()
                                    .into_query()
                                    .apply_prefix(base_table_prefix)
                                    .apply_order_by(&cursor_by, Order::Asc)
                                    .apply_filter(&cursor_by, from.clone(), |c, v| {
                                        Expr::col(SeaRc::clone(c)).gt(v)
                                    })
                                    .limit(limit + 1)
                                    .to_owned(),
                            )
                            .to_owned(),
                        Alias::new("page").into_iden(),
                    )
                    .to_owned(),
                Alias::new("cursored_page").into_iden(),
            )
            .apply_order_by(&prefixed_cursor_by, Order::Asc)
            .apply_filter(&prefixed_cursor_by, from.clone(), |c, v| {
                Expr::col(SeaRc::clone(c)).gt(v)
            })
            .limit(limit)
            .to_owned();

        let statement = db.get_database_backend().build(&stmt);
        let query_results = db.query_all(statement).await?;
        let neighbours = Neighbours::from_query_result(&query_results[0], NEIGHBOURS_PREFIX)?;
        let items = query_results
            .into_iter()
            .map(|query_result| E::Model::from_query_result(&query_result, base_table_prefix))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CursorPage {
            items,
            has_next: neighbours.has_next,
            has_previous: neighbours.has_previous,
        })
    }
}

trait ApplyFilter {
    fn apply_filter<Filter>(
        &mut self,
        columns: &[DynIden],
        values: Option<Values>,
        f: Filter,
    ) -> &mut Self
    where
        Filter: Fn(&DynIden, Value) -> SimpleExpr;
}

impl ApplyFilter for SelectStatement {
    /// Derived from `apply_filter` in [`sea_orm::Cursor`]
    /// See: <https://github.com/SeaQL/sea-orm/blob/c69b995800684b3f29eedba289a7e041fc54d328/src/executor/cursor.rs#L69>
    fn apply_filter<Filter>(
        &mut self,
        columns: &[DynIden],
        values: Option<Values>,
        filter_expr: Filter,
    ) -> &mut Self
    where
        Filter: Fn(&DynIden, Value) -> SimpleExpr,
    {
        if let Some(values) = values {
            let condition = (1..=columns.len())
                .rev()
                .fold(Condition::any(), |cond_any, n| {
                    let inner_cond_all =
                        columns.iter().zip(values.iter()).enumerate().take(n).fold(
                            Condition::all(),
                            |inner_cond_all, (i, (col, val))| {
                                let val = val.clone();

                                let expr = if i != (n - 1) {
                                    Expr::col(SeaRc::clone(col)).eq(val)
                                } else {
                                    filter_expr(col, val)
                                };
                                inner_cond_all.add(expr)
                            },
                        );
                    cond_any.add(inner_cond_all)
                });

            self.cond_where(condition);
        }
        self
    }
}

trait ApplyOrderBy {
    fn apply_order_by(&mut self, columns: &[DynIden], ord: Order) -> &mut Self;
}

impl<O: OrderedStatement> ApplyOrderBy for O {
    fn apply_order_by(&mut self, columns: &[DynIden], ord: Order) -> &mut Self {
        let order = |query: &mut O, col| {
            query.order_by(SeaRc::clone(col), ord.clone());
        };
        for column in columns {
            order(self, column)
        }
        self
    }
}

trait ApplyPrefix {
    fn apply_prefix(&mut self, pre: &str) -> &mut Self;
}

impl ApplyPrefix for SelectStatement {
    /// Derived from `apply_alias` in [`sea_orm::Select`]
    /// See: <https://github.com/SeaQL/sea-orm/blob/c69b995800684b3f29eedba289a7e041fc54d328/src/query/combine.rs#L35>
    fn apply_prefix(&mut self, pre: &str) -> &mut Self {
        self.exprs_mut_for_each(|sel| {
            match &sel.alias {
                Some(alias) => {
                    let alias = format!("{}{}", pre, alias.to_string().as_str());
                    sel.alias = Some(SeaRc::new(Alias::new(&alias)));
                }
                None => {
                    let col = match &sel.expr {
                        SimpleExpr::Column(col_ref) => match &col_ref {
                            ColumnRef::Column(col)
                            | ColumnRef::TableColumn(_, col)
                            | ColumnRef::SchemaTableColumn(_, _, col) => col,
                            ColumnRef::Asterisk | ColumnRef::TableAsterisk(_) => {
                                panic!("cannot apply alias for Column with asterisk")
                            }
                        },
                        SimpleExpr::AsEnum(_, simple_expr) => match simple_expr.as_ref() {
                            SimpleExpr::Column(col_ref) => match &col_ref {
                                ColumnRef::Column(col)
                                | ColumnRef::TableColumn(_, col)
                                | ColumnRef::SchemaTableColumn(_, _, col) => col,
                                ColumnRef::Asterisk | ColumnRef::TableAsterisk(_) => {
                                    panic!("cannot apply alias for AsEnum with asterisk")
                                }
                            },
                            _ => {
                                panic!("cannot apply alias for AsEnum with expr other than Column")
                            }
                        },
                        _ => panic!("cannot apply alias for expr other than Column or AsEnum"),
                    };
                    let alias = format!("{}{}", pre, col.to_string().as_str());
                    sel.alias = Some(SeaRc::new(Alias::new(&alias)));
                }
            };
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::QueryCursorPage;
    use crate::CursorPage;
    use sea_orm::{MockDatabase, Value, Values};

    mod table {
        use super::result_table;
        use sea_orm::{
            ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
            EnumIter, PrimaryKeyTrait,
        };

        #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "table")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: u64,
        }

        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}

        impl From<result_table::Model> for Model {
            fn from(value: result_table::Model) -> Self {
                Self { id: value.book_id }
            }
        }
    }

    mod result_table {
        use sea_orm::{
            ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
            EnumIter, PrimaryKeyTrait,
        };

        #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "table")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub book_id: u64,
            pub neighbours_has_previous: bool,
            pub neighbours_has_next: bool,
        }

        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[tokio::test]
    async fn page_after_start() {
        let models = vec![
            result_table::Model {
                book_id: 1,
                neighbours_has_previous: false,
                neighbours_has_next: true,
            },
            result_table::Model {
                book_id: 2,
                neighbours_has_previous: true,
                neighbours_has_next: false,
            },
            result_table::Model {
                book_id: 4,
                neighbours_has_previous: true,
                neighbours_has_next: false,
            },
        ];
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        let page = table::Entity::page_after(None, 3, &db).await.unwrap();

        assert_eq!(
            CursorPage {
                items: models.into_iter().map(table::Model::from).collect(),
                has_next: true,
                has_previous: false
            },
            page
        );
    }

    #[tokio::test]
    async fn page_after_cursor() {
        let models = vec![
            result_table::Model {
                book_id: 33,
                neighbours_has_next: false,
                neighbours_has_previous: true,
            },
            result_table::Model {
                book_id: 35,
                neighbours_has_next: false,
                neighbours_has_previous: true,
            },
            result_table::Model {
                book_id: 38,
                neighbours_has_next: false,
                neighbours_has_previous: true,
            },
        ];
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        let page = table::Entity::page_after(Some(Values(vec![Value::from(32)])), 3, &db)
            .await
            .unwrap();

        assert_eq!(
            CursorPage {
                has_next: false,
                has_previous: true,
                items: models
                    .into_iter()
                    .map(table::Model::from)
                    .collect::<Vec<_>>(),
            },
            page
        );
    }
}
