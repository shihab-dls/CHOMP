use cynic::impl_scalar;
use url::Url;
use uuid::Uuid;

/// Generated schema interface for the targeting service
#[cynic::schema("targeting")]
#[allow(clippy::missing_docs_in_private_items)]
pub mod targeting {}

impl_scalar!(Uuid, targeting::UUID);
impl_scalar!(Url, targeting::Url);
