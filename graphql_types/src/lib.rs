use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(Debug, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "WellInput")]
pub struct Well {
    pub plate: Uuid,
    pub well: i16,
}
