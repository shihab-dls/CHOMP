use crate::queries::create_prediction::{CreatePredictionMutation, CreatePredictionVariables};
use chimp_protocol::Response;
use cynic::{http::ReqwestExt, MutationBuilder};
use reqwest::Method;
use url::Url;

/// Recieves CHiMP predictions and sends them to the targeting service.
pub async fn handle_new_prediction(
    prediction: Result<Response, anyhow::Error>,
    targeting_client: reqwest::Client,
    targeting_url: Url,
    authorization_token: &str,
) -> Result<(), anyhow::Error> {
    if let Response::Success(succesful_response) = prediction? {
        let variables = CreatePredictionVariables::from(succesful_response);
        let mutation = CreatePredictionMutation::build(variables);
        let response = targeting_client
            .request(Method::POST, targeting_url)
            .header("Authorization", format!("Bearer {authorization_token}"))
            .run_graphql(mutation)
            .await
            .unwrap();
        if let Some(errs) = response.errors {
            panic!("Targeting service returned error(s): {errs:?}");
        }
    }
    Ok(())
}
