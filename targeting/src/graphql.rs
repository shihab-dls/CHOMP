use crate::resolvers::{
    image::{ImageMutation, ImageQuery, ImageSubscription},
    prediction::{PredicitonMutation, PredictionQuery},
};
use async_graphql::{MergedObject, MergedSubscription, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<RootQuery, RootMutation, RootSubscription> {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        RootSubscription::default(),
    )
}

pub type RootSchema = Schema<RootQuery, RootMutation, RootSubscription>;

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootQuery(ImageQuery, PredictionQuery);

#[derive(Debug, Clone, Default, MergedObject)]
pub struct RootMutation(ImageMutation, PredicitonMutation);

#[derive(Debug, Clone, Default, MergedSubscription)]
pub struct RootSubscription(ImageSubscription);
