#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use async_trait::async_trait;
use sea_orm::{
    sea_query::{
        Alias, ColumnRef, CommonTableExpression, Expr, IntoColumnRef, IntoIden, IntoValueTuple,
        Query, QueryStatementBuilder, SeaRc, SelectStatement, SimpleExpr, UnionType, ValueTuple,
        WindowStatement, WithClause,
    },
    Condition, ConnectionTrait, DbErr, DynIden, EntityTrait, FromQueryResult, Identity,
    IntoIdentity, Order, OrderedStatement, QueryTrait, Select, Statement, Value,
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
            has_previous: res.try_get::<Option<bool>>(pre, HAS_PREVIOUS)?.is_some(),
            has_next: res.try_get::<Option<bool>>(pre, HAS_NEXT)?.is_some(),
        })
    }
}

/// Allows for querying of pages from a selection.
#[async_trait]
pub trait QueryCursorPage {
    /// The type of item returned as page contents.
    type Item;

    /// Get a page of limited size after the cursor.
    async fn page_after<Columns, Cursor, DbConn>(
        self,
        columns: Columns,
        cursor: Option<Cursor>,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        Columns: IntoIdentity + Send + Sync,
        Cursor: IntoValueTuple + Clone + Send + Sync,
        DbConn: ConnectionTrait;
}

#[async_trait]
impl<E, M> QueryCursorPage for Select<E>
where
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    type Item = M;

    async fn page_after<Columns, Cursor, DbConn>(
        self,
        by: Columns,
        from: Option<Cursor>,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        Columns: IntoIdentity + Send + Sync,
        Cursor: IntoValueTuple + Clone + Send + Sync,
        DbConn: ConnectionTrait,
    {
        const BASE_TABLE: &str = "book";
        let base_table_prefix = format!("{BASE_TABLE}_");
        let base_table_iden = Alias::new(BASE_TABLE).into_iden();

        let base_query = self
            .into_query()
            .apply_prefix(&base_table_prefix)
            .to_owned();

        let with_base_query = WithClause::new()
            .cte(
                CommonTableExpression::new()
                    .query(base_query)
                    .table_name(base_table_iden.clone())
                    .to_owned(),
            )
            .to_owned();

        let by = by
            .into_identity()
            .apply_prefix(&base_table_prefix)
            .to_owned();

        let before_table_iden = Alias::new("before").into_iden();
        let page_table_iden = Alias::new("page").into_iden();
        let cursored_page_table_iden = Alias::new("cursored_page").into_iden();
        let stmt = Query::select()
            .column(ColumnRef::Asterisk)
            .from_subquery(
                Query::select()
                    .column(ColumnRef::Asterisk)
                    .expr_window_as(
                        Expr::cust_with_values("LAG(TRUE, $1)", [1_i32]),
                        WindowStatement::new()
                            .apply_order_by(by.clone(), None, Order::Asc)
                            .to_owned(),
                        Alias::new(&format!("{NEIGHBOURS_PREFIX}{HAS_PREVIOUS}")),
                    )
                    .expr_window_as(
                        Expr::cust_with_values("LEAD(TRUE, $1)", [limit as i32]),
                        WindowStatement::new()
                            .apply_order_by(by.clone(), None, Order::Asc)
                            .to_owned(),
                        Alias::new(&format!("{NEIGHBOURS_PREFIX}{HAS_NEXT}")),
                    )
                    .from_subquery(
                        Query::select()
                            .column(ColumnRef::Asterisk)
                            .from_subquery(
                                Query::select()
                                    .column(ColumnRef::Asterisk)
                                    .from(base_table_iden.clone())
                                    .apply_order_by(
                                        by.clone(),
                                        Some(base_table_iden.clone()),
                                        Order::Desc,
                                    )
                                    .apply_filter(
                                        by.clone(),
                                        from.clone(),
                                        base_table_iden.clone(),
                                        |c, v| {
                                            Expr::col((base_table_iden.clone(), SeaRc::clone(c)))
                                                .lte(v)
                                        },
                                    )
                                    .limit(1)
                                    .to_owned(),
                                before_table_iden.clone(),
                            )
                            .union(
                                UnionType::All,
                                Query::select()
                                    .column(ColumnRef::Asterisk)
                                    .from(base_table_iden.clone())
                                    .apply_order_by(
                                        by.clone(),
                                        Some(base_table_iden.clone()),
                                        Order::Asc,
                                    )
                                    .apply_filter(
                                        by.clone(),
                                        from.clone(),
                                        base_table_iden.clone(),
                                        |c, v| {
                                            Expr::col((base_table_iden.clone(), SeaRc::clone(c)))
                                                .gt(v)
                                        },
                                    )
                                    .limit(limit + 1)
                                    .to_owned(),
                            )
                            .to_owned(),
                        page_table_iden.clone(),
                    )
                    .to_owned(),
                cursored_page_table_iden.clone(),
            )
            .apply_order_by(
                by.clone(),
                Some(cursored_page_table_iden.clone()),
                Order::Asc,
            )
            .apply_filter(
                by.clone(),
                from.clone(),
                cursored_page_table_iden.clone(),
                |c, v| Expr::col((cursored_page_table_iden.clone(), SeaRc::clone(c))).gt(v),
            )
            .limit(limit)
            .to_owned()
            .with(with_base_query);

        let (sql, values) = stmt.build_any(db.get_database_backend().get_query_builder().as_ref());
        println!("{sql} with {values:?}");
        let statement = Statement {
            sql,
            values: Some(values),
            db_backend: db.get_database_backend(),
        };

        let query_results = db.query_all(statement).await?;
        let neighbours = Neighbours::from_query_result(&query_results[0], NEIGHBOURS_PREFIX)?;
        let items = query_results
            .into_iter()
            .map(|query_result| M::from_query_result(&query_result, &base_table_prefix))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CursorPage {
            items,
            has_next: neighbours.has_next,
            has_previous: neighbours.has_previous,
        })
    }
}

trait ApplyFilter {
    fn apply_filter<Cursor, Filter>(
        &mut self,
        columns: Identity,
        values: Option<Cursor>,
        table: DynIden,
        f: Filter,
    ) -> &mut Self
    where
        Cursor: IntoValueTuple,
        Filter: Fn(&DynIden, Value) -> SimpleExpr;
}

impl ApplyFilter for SelectStatement {
    /// Derived from `apply_filter` in [`sea_orm::Cursor`]
    /// See: <https://github.com/SeaQL/sea-orm/blob/c69b995800684b3f29eedba289a7e041fc54d328/src/executor/cursor.rs#L69>
    fn apply_filter<Cursor, Filter>(
        &mut self,
        columns: Identity,
        values: Option<Cursor>,
        table: DynIden,
        filter_expr: Filter,
    ) -> &mut Self
    where
        Cursor: IntoValueTuple,
        Filter: Fn(&DynIden, Value) -> SimpleExpr,
    {
        let condition = match (&columns, values.map(IntoValueTuple::into_value_tuple)) {
            (_, None) => Condition::all(),
            (Identity::Unary(c1), Some(ValueTuple::One(v1))) => {
                Condition::all().add(filter_expr(c1, v1))
            }
            (Identity::Binary(c1, c2), Some(ValueTuple::Two(v1, v2))) => Condition::any()
                .add(
                    Condition::all()
                        .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                        .add(filter_expr(c2, v2)),
                )
                .add(filter_expr(c1, v1)),
            (Identity::Ternary(c1, c2, c3), Some(ValueTuple::Three(v1, v2, v3))) => {
                Condition::any()
                    .add(
                        Condition::all()
                            .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                            .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c2))).eq(v2.clone()))
                            .add(filter_expr(c3, v3)),
                    )
                    .add(
                        Condition::all()
                            .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                            .add(filter_expr(c2, v2)),
                    )
                    .add(filter_expr(c1, v1))
            }
            _ => panic!("column arity mismatch"),
        };

        self.cond_where(condition)
    }
}

trait ApplyOrderBy {
    fn apply_order_by(
        &mut self,
        columns: Identity,
        table: Option<DynIden>,
        ord: Order,
    ) -> &mut Self;
}

impl<O: OrderedStatement> ApplyOrderBy for O {
    fn apply_order_by(
        &mut self,
        columns: Identity,
        table: Option<DynIden>,
        ord: Order,
    ) -> &mut Self {
        let order = |query: &mut O, col| {
            let column_ref = if let Some(table) = table.as_ref() {
                (SeaRc::clone(table), SeaRc::clone(col)).into_column_ref()
            } else {
                SeaRc::clone(col).into_column_ref()
            };
            query.order_by(column_ref, ord.clone());
        };
        match &columns {
            Identity::Unary(c1) => {
                order(self, c1);
            }
            Identity::Binary(c1, c2) => {
                order(self, c1);
                order(self, c2);
            }
            Identity::Ternary(c1, c2, c3) => {
                order(self, c1);
                order(self, c2);
                order(self, c3);
            }
        };
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

impl ApplyPrefix for Identity {
    fn apply_prefix(&mut self, pre: &str) -> &mut Self {
        match self {
            Identity::Unary(iden) => {
                *iden = Alias::new(&format!("{pre}{}", iden.to_string())).into_iden()
            }
            Identity::Binary(iden1, iden2) => {
                *iden1 = Alias::new(&format!("{pre}{}", iden1.to_string())).into_iden();
                *iden2 = Alias::new(&format!("{pre}{}", iden2.to_string())).into_iden();
            }
            Identity::Ternary(iden1, iden2, iden3) => {
                *iden1 = Alias::new(&format!("{pre}{}", iden1.to_string())).into_iden();
                *iden2 = Alias::new(&format!("{pre}{}", iden2.to_string())).into_iden();
                *iden3 = Alias::new(&format!("{pre}{}", iden3.to_string())).into_iden();
            }
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{CursorPage, QueryCursorPage};
    use sea_orm::{EntityTrait, MockDatabase};

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
            pub neighbours_has_previous: Option<bool>,
            pub neighbours_has_next: Option<bool>,
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
                neighbours_has_previous: None,
                neighbours_has_next: Some(true),
            },
            result_table::Model {
                book_id: 2,
                neighbours_has_previous: Some(true),
                neighbours_has_next: None,
            },
            result_table::Model {
                book_id: 4,
                neighbours_has_previous: Some(true),
                neighbours_has_next: None,
            },
        ];
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        let page = table::Entity::find()
            .page_after(table::Column::Id, None::<String>, 3, &db)
            .await
            .unwrap();

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
                neighbours_has_next: None,
                neighbours_has_previous: Some(true),
            },
            result_table::Model {
                book_id: 35,
                neighbours_has_next: None,
                neighbours_has_previous: Some(true),
            },
            result_table::Model {
                book_id: 38,
                neighbours_has_next: None,
                neighbours_has_previous: Some(true),
            },
        ];
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        let page = table::Entity::find()
            .page_after(table::Column::Id, Some(32), 3, &db)
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
