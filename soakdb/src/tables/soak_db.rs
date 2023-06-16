use crate::datatypes::{
    combination::AsSelfOr,
    text::{AsSelfOrText, NullAsVarious},
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "soakDB")]
pub struct Model {
    #[sea_orm(column_name = "rowid", primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(column_name = "Version", column_type = "Double", nullable)]
    pub version: Option<f64>,
    #[sea_orm(column_name = "LabVisit")]
    pub lab_visit: Option<String>,
    #[sea_orm(column_name = "Path")]
    pub path: Option<String>,
    #[sea_orm(column_name = "Protein")]
    pub protein: Option<String>,
    #[sea_orm(column_name = "DropVolume", column_type = "Double", nullable)]
    pub drop_volume: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CrystalsPerBatch")]
    pub crystals_per_batch: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(column_name = "OneBatchPerPlate")]
    pub one_batch_per_plate: Option<String>,
    #[sea_orm(column_name = "CompoundStock", column_type = "Double", nullable)]
    pub compound_stock: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "SolventPercent", column_type = "Double", nullable)]
    pub solvent_percent: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CryoStock", column_type = "Double", nullable)]
    pub cryo_stock: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "DesiredCryo", column_type = "Double", nullable)]
    pub desired_cryo: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CryoLocation")]
    pub cryo_location: Option<String>,
    #[sea_orm(column_name = "DesiredSoakTime", column_type = "Double", nullable)]
    pub desired_soak_time: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CrystalStartNumber")]
    pub crystal_start_number: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(column_name = "BeamlineVisit")]
    pub beamline_visit: Option<String>,
    #[sea_orm(column_name = "SpaceGroup")]
    pub space_group: Option<String>,
    #[sea_orm(column_name = "A", column_type = "Double", nullable)]
    pub a: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "B", column_type = "Double", nullable)]
    pub b: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "C", column_type = "Double", nullable)]
    pub c: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_type = "Double", nullable)]
    pub alpha: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_type = "Double", nullable)]
    pub beta: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_type = "Double", nullable)]
    pub gamma: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "Recipe")]
    pub recipe: Option<String>,
    #[sea_orm(column_name = "Resolution", column_type = "Double", nullable)]
    pub resolution: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CentringMethod")]
    pub centring_method: Option<String>,
    #[sea_orm(column_name = "EUBOpen")]
    pub eub_open: Option<String>,
    #[sea_orm(column_name = "iNEXT")]
    pub i_next: Option<String>,
    #[sea_orm(column_name = "Covid19")]
    pub covid19: Option<String>,
    #[sea_orm(column_name = "ILOXchem")]
    pub ilo_xchem: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::Entity;
    use crate::tests::connect_test_databases;
    use futures::{stream::FuturesOrdered, StreamExt};
    use sea_orm::EntityTrait;

    #[tokio::test]
    async fn read_from_test_database() {
        connect_test_databases()
            .map(|database| async {
                let (database, path) = database.await;
                Entity::find()
                    .all(&database.connection)
                    .await
                    .map_err(|err| panic!("At {:?} with {}", path, err))
                    .unwrap();
            })
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await;
    }
}
