use async_graphql::{async_trait::async_trait, Context, Guard, Name, Value};
use serde::Serialize;

use crate::{AuthorizationToken, OPAClient};

/// An [`async_graphql`] Resolver [`Guard`] which queries OPA for an Authorization decision.
///
/// The [`Context`] must contain both a [`OPAClient`] and an [`AuthorizationToken`], if either of these are not found an [`Err`] will be returned.
///
/// # Examples
/// ```
/// use async_graphql::SimpleObject;
/// use opa_client::graphql::OPAGuard;
///
/// #[derive(SimpleObject)]
/// struct MyModel {
///     #[graphql(guard = "OPAGuard::new(\"my.opa.policy.allow\")")]
///     value: i32
/// }
///
/// ```
///
/// ```
/// use async_graphql::{Object};
/// use opa_client::graphql::OPAGuard;
///
/// struct Query;
///
/// #[Object]
/// impl Query {
///     #[graphql(guard = "OPAGuard::new(\"my.opa.policy.allow\")")]
///     async fn value(&self, value: i32) -> i32 {
///         value
///     }
/// }
/// ```
pub struct OPAGuard {
    endpoint: String,
}

impl OPAGuard {
    /// Constructs a new [`OPAGuard`] which will query the provided policy for a decision.
    pub fn new(policy: impl AsRef<str>) -> Self {
        Self {
            endpoint: policy.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct OPAGraphQLInput {
    field: String,
    arguments: Vec<(Name, Value)>,
    token: AuthorizationToken,
}

#[async_trait]
impl Guard for OPAGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let authz_client = ctx.data::<OPAClient>()?;
        let token = ctx.data::<AuthorizationToken>()?.clone();
        let field = ctx.field().name().to_string();
        let arguments = ctx.field().arguments()?;
        let input = OPAGraphQLInput {
            field,
            arguments,
            token,
        };
        authz_client
            .decide::<_, bool>(&self.endpoint, input)
            .await?
            .then_some(())
            .ok_or(async_graphql::Error::new("Unauthorized"))
    }
}
