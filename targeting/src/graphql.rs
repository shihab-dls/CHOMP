use crate::resolvers::{
    image::{ImageMutation, ImageQuery},
    prediction::{PredicitonMutation, PredictionQuery},
};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, EmptySubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
}

pub type RootSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootQuery(ImageQuery, PredictionQuery);

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootMutation(ImageMutation, PredicitonMutation);
