use crate::datatypes::{
    combination::{AsSelfOr, NeverRead},
    datetime::DateTimeAsVarious,
    duration::{DurationAsExcelFloat, DurationAsVarious},
    mounting_result::MountingResultAsText,
    status::StatusAsText,
    text::{AsSelfOrText, FloatAsScientificText, NullAsVarious},
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mainTable")]
pub struct Model {
    #[sea_orm(column_name = "ID", primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_name = "LabVisit")]
    pub lab_visit: Option<String>,
    #[sea_orm(column_name = "LibraryPlate")]
    pub library_plate: Option<String>,
    #[sea_orm(column_name = "SourceWell")]
    pub source_well: Option<String>,
    #[sea_orm(column_name = "LibraryName")]
    pub library_name: Option<String>,
    #[sea_orm(column_name = "CompoundSMILES")]
    pub compound_smiles: Option<String>,
    #[sea_orm(column_name = "CompoundCode")]
    pub compound_code: Option<String>,
    #[sea_orm(column_name = "CrystalPlate")]
    pub crystal_plate: Option<String>,
    #[sea_orm(column_name = "CrystalWell")]
    pub crystal_well: Option<String>,
    #[sea_orm(column_name = "EchoX")]
    pub echo_x: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "EchoY")]
    pub echo_y: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "DropVolume", column_type = "Double", nullable)]
    pub drop_volume: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "ProteinName")]
    pub protein_name: Option<String>,
    #[sea_orm(column_name = "BatchNumber")]
    pub batch_number: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(
        column_name = "CompoundStockConcentration",
        column_type = "Double",
        nullable
    )]
    pub compound_stock_concentration: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(
        column_name = "CompoundConcentration",
        column_type = "Double",
        nullable
    )]
    pub compound_concentration: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "SolventFraction", column_type = "Double", nullable)]
    pub solvent_fraction: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "SoakTransferVol", column_type = "Double", nullable)]
    pub soak_transfer_vol: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "SoakStatus")]
    pub soak_status: Option<NullAsVarious<StatusAsText>>,
    #[sea_orm(column_name = "SoakTimestamp")]
    pub soak_timestamp: Option<NullAsVarious<DateTimeAsVarious>>,
    #[sea_orm(column_name = "CryoStockFraction", column_type = "Double", nullable)]
    pub cryo_stock_fraction: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(column_name = "CryoFraction", column_type = "Double", nullable)]
    pub cryo_fraction: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(column_name = "CryoWell")]
    pub cryo_well: Option<String>,
    #[sea_orm(column_name = "CryoTransferVolume", column_type = "Double", nullable)]
    pub cryo_transfer_volume: Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>,
    #[sea_orm(column_name = "CryoStatus")]
    pub cryo_status: Option<NullAsVarious<StatusAsText>>,
    #[sea_orm(column_name = "CryoTimestamp")]
    pub cryo_timestamp: Option<NullAsVarious<DateTimeAsVarious>>,
    #[sea_orm(column_name = "SoakingTime", column_type = "Double", nullable)]
    pub soaking_time: Option<NeverRead<NullAsVarious<DurationAsVarious>>>,
    #[sea_orm(column_name = "HarvestStatus")]
    pub harvest_status: Option<NullAsVarious<StatusAsText>>,
    #[sea_orm(column_name = "CrystalName")]
    pub crystal_name: Option<String>,
    #[sea_orm(column_name = "Puck")]
    pub puck: Option<String>,
    #[sea_orm(column_name = "PuckPosition")]
    pub puck_position: Option<NullAsVarious<AsSelfOrText<i32>>>,
    #[sea_orm(column_name = "PinBarcode")]
    pub pin_barcode: Option<String>,
    #[sea_orm(column_name = "MountingResult")]
    pub mounting_result: Option<NullAsVarious<MountingResultAsText>>,
    #[sea_orm(column_name = "MountingArrivalTime")]
    pub mounting_arrival_time: Option<NullAsVarious<DateTimeAsVarious>>,
    #[sea_orm(column_name = "MountedTimestamp")]
    pub mounted_timestamp: Option<NullAsVarious<DateTimeAsVarious>>,
    #[sea_orm(column_name = "MountingTime")]
    pub mounting_time:
        Option<NeverRead<NullAsVarious<FloatAsScientificText<DurationAsExcelFloat>>>>,
    #[sea_orm(column_name = "ispybStatus")]
    pub ispyb_status: Option<String>,
    #[sea_orm(column_name = "DataCollectionVisit")]
    pub data_collection_visit: Option<String>,
    #[sea_orm(column_name = "SoakDBComments")]
    pub soak_db_comments: Option<String>,
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
                    .all(&database)
                    .await
                    .map_err(|err| panic!("At {:?} with {}", path, err))
                    .unwrap();
            })
            .collect::<FuturesOrdered<_>>()
            .collect::<Vec<_>>()
            .await;
    }
}
