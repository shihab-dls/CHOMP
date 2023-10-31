use crate::{
    resolvers::CursorInput,
    tables::{
        pin_library::{self, PinStatus},
        pin_mount,
    },
};
use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    ComplexObject, Context, Object,
};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait};

#[ComplexObject]
impl pin_library::Model {
    async fn mounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_pin_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinLibraryQuery;

#[Object]
impl PinLibraryQuery {
    async fn library_pins(
        &self,
        ctx: &Context<'_>,
        cursor: CursorInput,
    ) -> async_graphql::Result<Connection<String, pin_library::Model, EmptyFields, EmptyFields>>
    {
        subject_authorization!("xchemlab.pin_packing.read_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;

        let page = cursor
            .into_query_cursor::<pin_library::Entity>()?
            .all(database)
            .await?;

        let mut connection = Connection::new(page.has_previous, page.has_next);
        connection.edges.extend(
            page.items
                .into_iter()
                .map(|pin| Edge::new(pin.barcode.clone(), pin)),
        );
        Ok(connection)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinLibraryMutation;

#[Object]
impl PinLibraryMutation {
    async fn register_library_pin(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        #[graphql(desc = "Mounting loop size in micrometers")] loop_size: i16,
    ) -> async_graphql::Result<pin_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let pin = pin_library::ActiveModel {
            barcode: ActiveValue::Set(barcode),
            loop_size: ActiveValue::Set(loop_size),
            status: ActiveValue::Set(PinStatus::Ready),
        };
        Ok(pin_library::Entity::insert(pin)
            .exec_with_returning(database)
            .await?)
    }

    async fn update_library_pin_status(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        status: PinStatus,
    ) -> async_graphql::Result<pin_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_pin_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let mut pin = pin_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find pin with barcode '{barcode}'"))?
            .into_active_model();
        pin.status = ActiveValue::Set(status);
        Ok(pin_library::Entity::update(pin).exec(database).await?)
    }
}
