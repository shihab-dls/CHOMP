use crate::datatypes::{
    combination::{AsSelfOr, NeverRead},
    datetime::DateTimeAsEuroText,
    duration::{DurationAsExcelFloat, DurationAsText, DurationAsVarious},
    fallible::FallibleRead,
    ispyb_export::ISPyBExportAsText,
    mounting_result::MountingResultAsText,
    status::StatusAsText,
    text::{AsSelfOrText, FloatAsScientificText, NullAsVarious},
    visit::VisitAsText,
};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct MetadataReadback {
    pub name: Option<String>,
    pub protein: Option<String>,
}

impl From<crate::tables::soak_db::Model> for MetadataReadback {
    fn from(value: crate::tables::soak_db::Model) -> Self {
        Self {
            protein: value.protein,
            name: value.lab_visit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Well {
    pub lab_visit: Visit,
    pub collection_visit: Visit,
    pub batch: i32,
    pub crystal: Crystal,
    pub solvent: Solvent,
    pub cryo: Cryo,
    pub mount: Mount,
    pub ispyb_export: ISPyBExport,
    pub comments: String,
}

#[derive(Debug, Clone)]
pub struct Visit {
    pub proposal_type: [char; 2],
    pub proposal_number: u32,
    pub visit_number: u32,
}

impl From<VisitAsText> for Visit {
    fn from(value: VisitAsText) -> Self {
        Self {
            proposal_type: value.proposal_type,
            proposal_number: value.proposal_number,
            visit_number: value.visit_number,
        }
    }
}

impl From<Visit> for VisitAsText {
    fn from(value: Visit) -> Self {
        Self {
            proposal_type: value.proposal_type,
            proposal_number: value.proposal_number,
            visit_number: value.visit_number,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Crystal {
    pub plate: String,
    pub well: String,
    pub name: String,
    pub position: Position,
    pub drop_volume: f64,
    pub protein_name: String,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Solvent {
    pub plate: String,
    pub well: String,
    pub name: String,
    pub smiles: String,
    pub code: String,
    pub stock_concentration: f64,
    pub concentration: f64,
    pub fraction: f64,
    pub transfer_volume: f64,
    pub status: Status,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Cryo {
    pub well: String,
    pub stock_fraction: i32,
    pub fraction: i32,
    pub transfer_volume: f64,
    pub status: Status,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Mount {
    pub puck_barcode: String,
    pub puck_well: i32,
    pub pin_barcode: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub harvest_status: Status,
    pub result: MountingResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

impl From<StatusAsText> for Status {
    fn from(value: StatusAsText) -> Self {
        match value {
            StatusAsText::Success => Self::Success,
            StatusAsText::Failure => Self::Failure,
            StatusAsText::Pending => Self::Pending,
        }
    }
}

impl From<Status> for StatusAsText {
    fn from(value: Status) -> Self {
        match value {
            Status::Success => StatusAsText::Success,
            Status::Failure => StatusAsText::Failure,
            Status::Pending => StatusAsText::Pending,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MountingResult {
    pub success: bool,
    pub comment_1: String,
    pub comment_2: String,
}

impl From<MountingResultAsText> for MountingResult {
    fn from(value: MountingResultAsText) -> Self {
        Self {
            success: value.success,
            comment_1: value.comment_1,
            comment_2: value.comment_2,
        }
    }
}

impl From<MountingResult> for MountingResultAsText {
    fn from(value: MountingResult) -> Self {
        Self {
            success: value.success,
            comment_1: value.comment_1,
            comment_2: value.comment_2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ISPyBExport {
    Exported,
    Pending,
}

impl From<ISPyBExportAsText> for ISPyBExport {
    fn from(value: ISPyBExportAsText) -> Self {
        match value {
            ISPyBExportAsText::Exported => Self::Exported,
            ISPyBExportAsText::ExportedTo(_) => Self::Exported,
            ISPyBExportAsText::Pending => Self::Pending,
        }
    }
}

impl From<ISPyBExport> for ISPyBExportAsText {
    fn from(value: ISPyBExport) -> Self {
        match value {
            ISPyBExport::Exported => Self::Exported,
            ISPyBExport::Pending => Self::Pending,
        }
    }
}

impl From<Well> for crate::tables::main_table::ActiveModel {
    fn from(value: Well) -> Self {
        Self {
            id: ActiveValue::NotSet,
            lab_visit: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                VisitAsText::from(value.lab_visit),
            )))),
            library_plate: ActiveValue::Set(FallibleRead::Ok(Some(value.solvent.plate))),
            source_well: ActiveValue::Set(FallibleRead::Ok(Some(value.solvent.well))),
            library_name: ActiveValue::Set(FallibleRead::Ok(Some(value.solvent.name))),
            compound_smiles: ActiveValue::Set(FallibleRead::Ok(Some(value.solvent.smiles))),
            compound_code: ActiveValue::Set(FallibleRead::Ok(Some(value.solvent.code))),
            crystal_plate: ActiveValue::Set(FallibleRead::Ok(Some(value.crystal.plate))),
            crystal_well: ActiveValue::Set(FallibleRead::Ok(Some(value.crystal.well))),
            echo_x: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.crystal.position.x)),
            )))),
            echo_y: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.crystal.position.y)),
            )))),
            drop_volume: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.crystal.drop_volume)),
            )))),
            protein_name: ActiveValue::Set(FallibleRead::Ok(Some(value.crystal.protein_name))),
            batch_number: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(value.batch),
            )))),
            compound_stock_concentration: ActiveValue::Set(FallibleRead::Ok(Some(
                NullAsVarious::from(AsSelfOrText::from(AsSelfOr::from(
                    value.solvent.stock_concentration,
                ))),
            ))),
            compound_concentration: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.solvent.concentration)),
            )))),
            solvent_fraction: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.solvent.fraction)),
            )))),
            soak_transfer_vol: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.solvent.transfer_volume)),
            )))),
            soak_status: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                StatusAsText::from(value.solvent.status),
            )))),
            soak_timestamp: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOr::from(DateTimeAsEuroText::from(value.solvent.timestamp)),
            )))),
            cryo_stock_fraction: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(value.cryo.stock_fraction),
            )))),
            cryo_fraction: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(value.cryo.fraction),
            )))),
            cryo_well: ActiveValue::Set(FallibleRead::Ok(Some(value.cryo.well))),
            cryo_transfer_volume: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(AsSelfOr::from(value.cryo.transfer_volume)),
            )))),
            cryo_status: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                StatusAsText::from(value.cryo.status),
            )))),
            cryo_timestamp: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOr::from(DateTimeAsEuroText::from(value.cryo.timestamp)),
            )))),
            soaking_time: ActiveValue::Set(FallibleRead::Ok(Some(NeverRead::from(
                NullAsVarious::from(DurationAsVarious::from(DurationAsText::from(
                    value.mount.end_time - value.solvent.timestamp,
                ))),
            )))),
            harvest_status: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                StatusAsText::from(value.mount.harvest_status),
            )))),
            crystal_name: ActiveValue::Set(FallibleRead::Ok(Some(value.crystal.name))),
            puck: ActiveValue::Set(FallibleRead::Ok(Some(value.mount.puck_barcode))),
            puck_position: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOrText::from(value.mount.puck_well),
            )))),
            pin_barcode: ActiveValue::Set(FallibleRead::Ok(Some(value.mount.pin_barcode))),
            mounting_result: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                MountingResultAsText::from(value.mount.result),
            )))),
            mounting_arrival_time: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOr::from(DateTimeAsEuroText::from(value.mount.start_time)),
            )))),
            mounted_timestamp: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                AsSelfOr::from(DateTimeAsEuroText::from(value.mount.end_time)),
            )))),
            mounting_time: ActiveValue::Set(FallibleRead::Ok(Some(NeverRead::from(
                NullAsVarious::from(FloatAsScientificText::from(DurationAsExcelFloat::from(
                    value.mount.end_time - value.mount.start_time,
                ))),
            )))),
            ispyb_status: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                ISPyBExportAsText::from(value.ispyb_export),
            )))),
            data_collection_visit: ActiveValue::Set(FallibleRead::Ok(Some(NullAsVarious::from(
                VisitAsText::from(value.collection_visit),
            )))),
            soak_db_comments: ActiveValue::Set(FallibleRead::Ok(Some(value.comments))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WellReadback {
    pub id: i32,
    pub lab_visit_name: Fallible<Option<Visit>>,
    pub collection_visit_name: Fallible<Option<Visit>>,
    pub batch: Fallible<Option<i32>>,
    pub crystal: CrystalReadback,
    pub solvent: SolventReadback,
    pub cryo: CryoReadback,
    pub mount: MountReadback,
    pub ispyb_export: Fallible<Option<ISPyBExport>>,
    pub comments: Fallible<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct CrystalReadback {
    pub plate: Fallible<Option<String>>,
    pub well: Fallible<Option<String>>,
    pub name: Fallible<Option<String>>,
    pub position: Fallible<Option<Position>>,
    pub drop_volume: Fallible<Option<f64>>,
    pub protein_name: Fallible<Option<String>>,
}

#[derive(Debug, Clone)]
pub enum Fallible<T> {
    Ok(T),
    Fail(String),
}

impl<T> Fallible<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Fallible<U> {
        match self {
            Self::Ok(t) => Fallible::Ok(op(t)),
            Self::Fail(e) => Fallible::Fail(e),
        }
    }
}

impl<T> Fallible<Option<T>> {
    pub fn map_opt<U, F: FnOnce(T) -> U>(self, op: F) -> Fallible<Option<U>> {
        match self {
            Self::Ok(option) => Fallible::Ok(option.map(op)),
            Self::Fail(unparsable) => Fallible::Fail(unparsable),
        }
    }
}

impl<T> From<FallibleRead<T>> for Fallible<T>
where
    sea_orm::Value: From<T>,
    T: sea_orm::TryGetable + sea_orm::sea_query::ValueType,
{
    fn from(value: FallibleRead<T>) -> Self {
        match value {
            FallibleRead::Ok(val) => Self::Ok(val),
            FallibleRead::Fail(unparsable) => Self::Fail(unparsable),
        }
    }
}

impl<T> From<Fallible<T>> for FallibleRead<T>
where
    sea_orm::Value: From<T>,
    T: sea_orm::TryGetable + sea_orm::sea_query::ValueType,
{
    fn from(value: Fallible<T>) -> Self {
        match value {
            Fallible::Ok(val) => Self::Ok(val),
            Fallible::Fail(unparsable) => Self::Fail(unparsable),
        }
    }
}

impl<T> From<Fallible<T>> for Result<T, String> {
    fn from(value: Fallible<T>) -> Self {
        match value {
            Fallible::Ok(val) => Result::Ok(val),
            Fallible::Fail(unparsable) => Result::Err(unparsable),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolventReadback {
    pub plate: Fallible<Option<String>>,
    pub well: Fallible<Option<String>>,
    pub name: Fallible<Option<String>>,
    pub smiles: Fallible<Option<String>>,
    pub code: Fallible<Option<String>>,
    pub stock_concentration: Fallible<Option<f64>>,
    pub concentration: Fallible<Option<f64>>,
    pub fraction: Fallible<Option<f64>>,
    pub transfer_volume: Fallible<Option<f64>>,
    pub status: Fallible<Option<Status>>,
    pub timestamp: Fallible<Option<DateTime<Utc>>>,
}

#[derive(Debug, Clone)]
pub struct CryoReadback {
    pub well: Fallible<Option<String>>,
    pub stock_fraction: Fallible<Option<i32>>,
    pub fraction: Fallible<Option<i32>>,
    pub transfer_volume: Fallible<Option<f64>>,
    pub status: Fallible<Option<Status>>,
    pub timestamp: Fallible<Option<DateTime<Utc>>>,
}

#[derive(Debug, Clone)]
pub struct MountReadback {
    pub puck_barcode: Fallible<Option<String>>,
    pub puck_well: Fallible<Option<i32>>,
    pub pin_barcode: Fallible<Option<String>>,
    pub start_time: Fallible<Option<DateTime<Utc>>>,
    pub end_time: Fallible<Option<DateTime<Utc>>>,
    pub harvest_status: Fallible<Option<Status>>,
    pub result: Fallible<Option<MountingResult>>,
}

impl From<crate::tables::main_table::Model> for WellReadback {
    fn from(value: crate::tables::main_table::Model) -> Self {
        let crystal_position = match (value.echo_x, value.echo_y) {
            (FallibleRead::Ok(Some(x)), FallibleRead::Ok(Some(y))) => {
                Fallible::Ok(Some(Position { x: ***x, y: ***y }))
            }
            (FallibleRead::Ok(None), _) => Fallible::Ok(None),
            (_, FallibleRead::Ok(None)) => Fallible::Ok(None),
            (FallibleRead::Fail(unparsable), _) => Fallible::Fail(unparsable),
            (_, FallibleRead::Fail(unparsable)) => Fallible::Fail(unparsable),
        };
        Self {
            id: value.id,
            lab_visit_name: Fallible::from(value.lab_visit)
                .map_opt(|val| Visit::from((*val).clone())),
            collection_visit_name: Fallible::from(value.data_collection_visit)
                .map_opt(|val| Visit::from((*val).clone())),
            batch: Fallible::from(value.batch_number).map_opt(|val| **val),
            crystal: CrystalReadback {
                plate: Fallible::from(value.crystal_plate),
                well: Fallible::from(value.crystal_well),
                name: Fallible::from(value.crystal_name),
                position: crystal_position,
                drop_volume: Fallible::from(value.drop_volume).map_opt(|val| ***val),
                protein_name: Fallible::from(value.protein_name),
            },
            solvent: SolventReadback {
                plate: Fallible::from(value.library_plate),
                well: Fallible::from(value.source_well),
                name: Fallible::from(value.library_name),
                smiles: Fallible::from(value.compound_smiles),
                code: Fallible::from(value.compound_code),
                stock_concentration: Fallible::from(value.compound_stock_concentration)
                    .map_opt(|val| ***val),
                concentration: Fallible::from(value.compound_concentration).map_opt(|val| ***val),
                fraction: Fallible::from(value.solvent_fraction).map_opt(|val| ***val),
                transfer_volume: Fallible::from(value.soak_transfer_vol).map_opt(|val| ***val),
                status: Fallible::from(value.soak_status).map_opt(|val| Status::from(*val)),
                timestamp: Fallible::from(value.soak_timestamp).map_opt(|val| ***val),
            },
            cryo: CryoReadback {
                well: Fallible::from(value.cryo_well),
                stock_fraction: Fallible::from(value.cryo_stock_fraction).map_opt(|val| **val),
                fraction: Fallible::from(value.cryo_fraction).map_opt(|val| **val),
                transfer_volume: Fallible::from(value.cryo_transfer_volume).map_opt(|val| ***val),
                status: Fallible::from(value.cryo_status).map_opt(|val| Status::from(*val)),
                timestamp: Fallible::from(value.cryo_timestamp).map_opt(|val| ***val),
            },
            mount: MountReadback {
                puck_barcode: Fallible::from(value.puck),
                puck_well: Fallible::from(value.puck_position).map_opt(|val| **val),
                pin_barcode: Fallible::from(value.pin_barcode),
                start_time: Fallible::from(value.mounting_arrival_time).map_opt(|val| ***val),
                end_time: Fallible::from(value.mounted_timestamp).map_opt(|val| ***val),
                harvest_status: Fallible::from(value.harvest_status)
                    .map_opt(|val| Status::from(*val)),
                result: Fallible::from(value.mounting_result)
                    .map_opt(|val| MountingResult::from((*val).clone())),
            },
            ispyb_export: Fallible::from(value.ispyb_status)
                .map_opt(|val| ISPyBExport::from((*val).clone())),
            comments: Fallible::from(value.soak_db_comments),
        }
    }
}
