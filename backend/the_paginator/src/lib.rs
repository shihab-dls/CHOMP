#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use async_trait::async_trait;
use sea_orm::{
    sea_query::{
        Alias, ColumnRef, CommonTableExpression, Expr, IntoIden, IntoValueTuple,
        QueryStatementBuilder, SeaRc, SelectStatement, SimpleExpr, ValueTuple, WithClause,
    },
    Condition, ConnectionTrait, DbErr, DynIden, EntityTrait, FromQueryResult, Identity,
    IntoIdentity, QueryTrait, Select, Statement, Value,
};

/// The contents of a cursor indexed page, with indicators for the existance of previous and next pages.
#[derive(Debug)]
pub struct CursorPage<I> {
    /// The rows found in the page
    pub items: Vec<I>,
    /// True if at least one row exists before this page
    pub has_previous: bool,
    /// True if at least one row exists after this page
    pub has_next: bool,
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
        cursor: Cursor,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        Columns: IntoIdentity + Send + Sync,
        Cursor: IntoValueTuple + Send + Sync,
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
        from: Cursor,
        limit: u64,
        db: &DbConn,
    ) -> Result<CursorPage<Self::Item>, DbErr>
    where
        Columns: IntoIdentity + Send + Sync,
        Cursor: IntoValueTuple + Send + Sync,
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
        let condition = build_condition(
            by,
            from.into_value_tuple(),
            base_table_iden.clone(),
            |c, v| Expr::col((base_table_iden.clone(), SeaRc::clone(c))).gt(v),
        );

        let stmt = SelectStatement::new()
            .column(ColumnRef::Asterisk)
            .from(base_table_iden)
            .cond_where(condition)
            .limit(limit)
            .to_owned()
            .with(with_base_query);

        let (sql, values) = stmt.build_any(db.get_database_backend().get_query_builder().as_ref());
        println!("{sql}");
        let statement = Statement {
            sql,
            values: Some(values),
            db_backend: db.get_database_backend(),
        };

        let query_results = db.query_all(statement).await?;
        let items = query_results
            .into_iter()
            .map(|query_result| M::from_query_result(&query_result, &base_table_prefix))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(CursorPage {
            items,
            has_next: false,
            has_previous: false,
        })
    }
}

/// Copied from `apply_filter` in [`sea_orm::Cursor`]
/// See: https://github.com/SeaQL/sea-orm/blob/c69b995800684b3f29eedba289a7e041fc54d328/src/executor/cursor.rs#L69
fn build_condition<V, F>(columns: Identity, values: V, table: DynIden, f: F) -> Condition
where
    V: IntoValueTuple,
    F: Fn(&DynIden, Value) -> SimpleExpr,
{
    match (&columns, values.into_value_tuple()) {
        (Identity::Unary(c1), ValueTuple::One(v1)) => Condition::all().add(f(c1, v1)),
        (Identity::Binary(c1, c2), ValueTuple::Two(v1, v2)) => Condition::any()
            .add(
                Condition::all()
                    .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                    .add(f(c2, v2)),
            )
            .add(f(c1, v1)),
        (Identity::Ternary(c1, c2, c3), ValueTuple::Three(v1, v2, v3)) => Condition::any()
            .add(
                Condition::all()
                    .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                    .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c2))).eq(v2.clone()))
                    .add(f(c3, v3)),
            )
            .add(
                Condition::all()
                    .add(Expr::col((SeaRc::clone(&table), SeaRc::clone(c1))).eq(v1.clone()))
                    .add(f(c2, v2)),
            )
            .add(f(c1, v1)),
        _ => panic!("column arity mismatch"),
    }
}

trait ApplyPrefix {
    fn apply_prefix(&mut self, pre: &str) -> &mut Self;
}

impl ApplyPrefix for SelectStatement {
    /// Copied from `apply_alias` in [`sea_orm::Select`]
    ///  https://github.com/SeaQL/sea-orm/blob/c69b995800684b3f29eedba289a7e041fc54d328/src/query/combine.rs#L35
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
    use crate::QueryCursorPage;
    use sea_orm::{EntityTrait, MockDatabase};

    mod table {
        use super::book_table;
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

        impl From<book_table::Model> for Model {
            fn from(value: book_table::Model) -> Self {
                Self { id: value.book_id }
            }
        }
    }

    mod book_table {
        use sea_orm::{
            ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
            EnumIter, PrimaryKeyTrait,
        };

        #[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
        #[sea_orm(table_name = "table")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub book_id: u64,
        }

        #[derive(Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    #[tokio::test]
    async fn page_after_simple() {
        let models = vec![
            book_table::Model { book_id: 33 },
            book_table::Model { book_id: 35 },
            book_table::Model { book_id: 38 },
        ];
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([models.clone()])
            .into_connection();

        let page = table::Entity::find()
            .page_after(table::Column::Id, 32, 3, &db)
            .await
            .unwrap();

        assert_eq!(
            models
                .into_iter()
                .map(table::Model::from)
                .collect::<Vec<_>>(),
            page.items
        );
    }
}
