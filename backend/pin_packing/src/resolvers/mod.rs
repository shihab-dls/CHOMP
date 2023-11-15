pub mod cane_library;
pub mod cane_mount;
pub mod crystal;
pub mod pin_library;
pub mod pin_mount;
pub mod puck_library;
pub mod puck_mount;

use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(Debug, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "WellInput")]
pub struct Well {
    pub plate: Uuid,
    pub well: i16,
}
