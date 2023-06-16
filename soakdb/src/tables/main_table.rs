use crate::datatypes::{
    combination::{AsSelfOr, NeverRead},
    datetime::DateTimeAsVarious,
    duration::{DurationAsExcelFloat, DurationAsVarious},
    fallible::FallibleRead,
    ispyb_export::ISPyBExportAsText,
    mounting_result::MountingResultAsText,
    status::StatusAsText,
    text::{AsSelfOrText, FloatAsScientificText, NullAsVarious},
    visit::VisitAsText,
};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mainTable")]
pub struct Model {
    #[sea_orm(column_name = "ID", primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_name = "LabVisit")]
    pub lab_visit: FallibleRead<Option<NullAsVarious<VisitAsText>>>,
    #[sea_orm(column_name = "LibraryPlate")]
    pub library_plate: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "SourceWell")]
    pub source_well: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "LibraryName")]
    pub library_name: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "CompoundSMILES")]
    pub compound_smiles: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "CompoundCode")]
    pub compound_code: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "CrystalPlate")]
    pub crystal_plate: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "CrystalWell")]
    pub crystal_well: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "EchoX")]
    #[allow(clippy::type_complexity)]
    pub echo_x: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "EchoY")]
    #[allow(clippy::type_complexity)]
    pub echo_y: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "DropVolume", column_type = "Double", nullable)]
    #[allow(clippy::type_complexity)]
    pub drop_volume: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "ProteinName")]
    pub protein_name: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "BatchNumber")]
    pub batch_number: FallibleRead<Option<NullAsVarious<AsSelfOrText<i32>>>>,
    #[sea_orm(
        column_name = "CompoundStockConcentration",
        column_type = "Double",
        nullable
    )]
    #[allow(clippy::type_complexity)]
    pub compound_stock_concentration:
        FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(
        column_name = "CompoundConcentration",
        column_type = "Double",
        nullable
    )]
    #[allow(clippy::type_complexity)]
    pub compound_concentration:
        FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "SolventFraction", column_type = "Double", nullable)]
    #[allow(clippy::type_complexity)]
    pub solvent_fraction: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "SoakTransferVol", column_type = "Double", nullable)]
    #[allow(clippy::type_complexity)]
    pub soak_transfer_vol: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "SoakStatus")]
    pub soak_status: FallibleRead<Option<NullAsVarious<StatusAsText>>>,
    #[sea_orm(column_name = "SoakTimestamp")]
    pub soak_timestamp: FallibleRead<Option<NullAsVarious<DateTimeAsVarious>>>,
    #[sea_orm(column_name = "CryoStockFraction", column_type = "Double", nullable)]
    pub cryo_stock_fraction: FallibleRead<Option<NullAsVarious<AsSelfOrText<i32>>>>,
    #[sea_orm(column_name = "CryoFraction", column_type = "Double", nullable)]
    pub cryo_fraction: FallibleRead<Option<NullAsVarious<AsSelfOrText<i32>>>>,
    #[sea_orm(column_name = "CryoWell")]
    pub cryo_well: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "CryoTransferVolume", column_type = "Double", nullable)]
    #[allow(clippy::type_complexity)]
    pub cryo_transfer_volume: FallibleRead<Option<NullAsVarious<AsSelfOrText<AsSelfOr<f64, i32>>>>>,
    #[sea_orm(column_name = "CryoStatus")]
    pub cryo_status: FallibleRead<Option<NullAsVarious<StatusAsText>>>,
    #[sea_orm(column_name = "CryoTimestamp")]
    pub cryo_timestamp: FallibleRead<Option<NullAsVarious<DateTimeAsVarious>>>,
    #[sea_orm(column_name = "SoakingTime", column_type = "Double", nullable)]
    pub soaking_time: FallibleRead<Option<NeverRead<NullAsVarious<DurationAsVarious>>>>,
    #[sea_orm(column_name = "HarvestStatus")]
    pub harvest_status: FallibleRead<Option<NullAsVarious<StatusAsText>>>,
    #[sea_orm(column_name = "CrystalName")]
    pub crystal_name: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "Puck")]
    pub puck: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "PuckPosition")]
    pub puck_position: FallibleRead<Option<NullAsVarious<AsSelfOrText<i32>>>>,
    #[sea_orm(column_name = "PinBarcode")]
    pub pin_barcode: FallibleRead<Option<String>>,
    #[sea_orm(column_name = "MountingResult")]
    pub mounting_result: FallibleRead<Option<NullAsVarious<MountingResultAsText>>>,
    #[sea_orm(column_name = "MountingArrivalTime")]
    pub mounting_arrival_time: FallibleRead<Option<NullAsVarious<DateTimeAsVarious>>>,
    #[sea_orm(column_name = "MountedTimestamp")]
    pub mounted_timestamp: FallibleRead<Option<NullAsVarious<DateTimeAsVarious>>>,
    #[sea_orm(column_name = "MountingTime")]
    pub mounting_time:
        FallibleRead<Option<NeverRead<NullAsVarious<FloatAsScientificText<DurationAsExcelFloat>>>>>,
    #[sea_orm(column_name = "ispybStatus")]
    pub ispyb_status: FallibleRead<Option<NullAsVarious<ISPyBExportAsText>>>,
    #[sea_orm(column_name = "DataCollectionVisit")]
    pub data_collection_visit: FallibleRead<Option<NullAsVarious<VisitAsText>>>,
    #[sea_orm(column_name = "SoakDBComments")]
    pub soak_db_comments: FallibleRead<Option<String>>,
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
