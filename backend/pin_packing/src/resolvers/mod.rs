pub mod cane_library;
pub mod cane_mount;
pub mod crystal;
pub mod pin_library;
pub mod pin_mount;
pub mod puck_library;
pub mod puck_mount;

use async_graphql::InputObject;

#[derive(Debug, Clone, InputObject)]
struct Cursor {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}
