use sea_orm::ActiveValue;

#[cfg(feature = "graphql-models")]
use async_graphql::{InputObject, SimpleObject};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Metadata {
    pub name: String,
    pub protein: String,
}

impl From<Metadata> for crate::tables::soak_db::ActiveModel {
    fn from(value: Metadata) -> Self {
        Self {
            id: ActiveValue::Set(1),
            version: ActiveValue::NotSet,
            lab_visit: ActiveValue::Set(Some(value.name)),
            path: ActiveValue::NotSet,
            protein: ActiveValue::Set(Some(value.protein)),
            drop_volume: ActiveValue::NotSet,
            crystals_per_batch: ActiveValue::NotSet,
            one_batch_per_plate: ActiveValue::NotSet,
            compound_stock: ActiveValue::NotSet,
            solvent_percent: ActiveValue::NotSet,
            cryo_stock: ActiveValue::NotSet,
            desired_cryo: ActiveValue::NotSet,
            cryo_location: ActiveValue::NotSet,
            desired_soak_time: ActiveValue::NotSet,
            crystal_start_number: ActiveValue::NotSet,
            beamline_visit: ActiveValue::NotSet,
            space_group: ActiveValue::NotSet,
            a: ActiveValue::NotSet,
            b: ActiveValue::NotSet,
            c: ActiveValue::NotSet,
            alpha: ActiveValue::NotSet,
            beta: ActiveValue::NotSet,
            gamma: ActiveValue::NotSet,
            recipe: ActiveValue::NotSet,
            resolution: ActiveValue::NotSet,
            centring_method: ActiveValue::NotSet,
            eub_open: ActiveValue::NotSet,
            i_next: ActiveValue::NotSet,
            covid19: ActiveValue::NotSet,
            ilo_xchem: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct MetadataReadback {
    pub name: Option<String>,
    pub protein: Option<String>,
}

impl From<Metadata> for MetadataReadback {
    fn from(value: Metadata) -> Self {
        Self {
            name: Some(value.name),
            protein: Some(value.protein),
        }
    }
}

impl From<crate::tables::soak_db::Model> for MetadataReadback {
    fn from(value: crate::tables::soak_db::Model) -> Self {
        Self {
            protein: value.protein,
            name: value.lab_visit,
        }
    }
}
