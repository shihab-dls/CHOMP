use crate::datatypes::{
    combination::{AsSelfOr, NeverRead},
    datetime::DateTimeAsEuroText,
    duration::{DurationAsExcelFloat, DurationAsText, DurationAsVarious},
    ispyb_export::ISPyBExportAsText,
    mounting_result::MountingResultAsText,
    status::StatusAsText,
    text::{
        AsSelfOrText, FloatAsScientificText, NullAsEmptyString, NullAsLiteralNa, NullAsLiteralNone,
    },
    visit::VisitAsText,
};
#[cfg(feature = "graphql-models")]
use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;

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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Well {
    lab_visit: Visit,
    collection_visit: Visit,
    batch: i32,
    crystal: Crystal,
    solvent: Solvent,
    cryo: Cryo,
    mount: Mount,
    ispyb_export: ISPyBExport,
    comments: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Visit {
    proposal_type: [char; 2],
    proposal_number: u32,
    visit_number: u32,
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
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Crystal {
    plate: String,
    well: String,
    name: String,
    position: Position,
    drop_volume: f64,
    protein_name: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Solvent {
    plate: String,
    well: String,
    name: String,
    smiles: String,
    code: String,
    stock_concentration: f64,
    concentration: f64,
    fraction: f64,
    transfer_volume: f64,
    status: Status,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Cryo {
    well: String,
    stock_fraction: i32,
    fraction: i32,
    transfer_volume: f64,
    status: Status,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct Mount {
    puck_barcode: String,
    puck_well: i32,
    pin_barcode: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    harvest_status: Status,
    result: MountingResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "graphql-models", derive(Enum))]
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

impl From<Status> for crate::datatypes::status::StatusAsText {
    fn from(value: Status) -> Self {
        match value {
            Status::Success => crate::datatypes::status::StatusAsText::Success,
            Status::Failure => crate::datatypes::status::StatusAsText::Failure,
            Status::Pending => crate::datatypes::status::StatusAsText::Pending,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject, InputObject))]
pub struct MountingResult {
    success: bool,
    comment_1: String,
    comment_2: String,
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
#[cfg_attr(feature = "graphql-models", derive(Enum))]
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
            lab_visit: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(VisitAsText::from(value.lab_visit)),
            )))),
            library_plate: ActiveValue::Set(Some(value.solvent.plate)),
            source_well: ActiveValue::Set(Some(value.solvent.well)),
            library_name: ActiveValue::Set(Some(value.solvent.name)),
            compound_smiles: ActiveValue::Set(Some(value.solvent.smiles)),
            compound_code: ActiveValue::Set(Some(value.solvent.code)),
            crystal_plate: ActiveValue::Set(Some(value.crystal.plate)),
            crystal_well: ActiveValue::Set(Some(value.crystal.well)),
            echo_x: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(value.crystal.position.x))),
            )))),
            echo_y: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(value.crystal.position.y))),
            )))),
            drop_volume: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.crystal.drop_volume,
                ))),
            )))),
            protein_name: ActiveValue::Set(Some(value.crystal.protein_name)),
            batch_number: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(AsSelfOrText::from(value.batch)),
            )))),
            compound_stock_concentration: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.solvent.stock_concentration,
                )))),
            ))),
            compound_concentration: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.solvent.concentration,
                )))),
            ))),
            solvent_fraction: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.solvent.fraction,
                )))),
            ))),
            soak_transfer_vol: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.solvent.transfer_volume,
                )))),
            ))),
            soak_status: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(StatusAsText::from(value.solvent.status)),
            )))),
            soak_timestamp: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOr::from(
                    DateTimeAsEuroText::from(value.solvent.timestamp),
                ))),
            ))),
            cryo_stock_fraction: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(
                    value.cryo.stock_fraction,
                ))),
            ))),
            cryo_fraction: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(
                    value.cryo.fraction,
                ))),
            ))),
            cryo_well: ActiveValue::Set(Some(value.cryo.well)),
            cryo_transfer_volume: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(AsSelfOr::from(
                    value.cryo.transfer_volume,
                )))),
            ))),
            cryo_status: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(StatusAsText::from(value.cryo.status)),
            )))),
            cryo_timestamp: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOr::from(
                    DateTimeAsEuroText::from(value.cryo.timestamp),
                ))),
            ))),
            soaking_time: ActiveValue::Set(Some(NeverRead::from(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(DurationAsVarious::from(
                    DurationAsText::from(value.mount.end_time - value.solvent.timestamp),
                ))),
            )))),
            harvest_status: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(StatusAsText::from(
                    value.mount.harvest_status,
                ))),
            ))),
            crystal_name: ActiveValue::Set(Some(value.crystal.name)),
            puck: ActiveValue::Set(Some(value.mount.puck_barcode)),
            puck_position: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOrText::from(
                    value.mount.puck_well,
                ))),
            ))),
            pin_barcode: ActiveValue::Set(Some(value.mount.pin_barcode)),
            mounting_result: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(MountingResultAsText::from(
                    value.mount.result,
                ))),
            ))),
            mounting_arrival_time: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOr::from(
                    DateTimeAsEuroText::from(value.mount.start_time),
                ))),
            ))),
            mounted_timestamp: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(AsSelfOr::from(
                    DateTimeAsEuroText::from(value.mount.end_time),
                ))),
            ))),
            mounting_time: ActiveValue::Set(Some(NeverRead::from(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(FloatAsScientificText::from(
                    DurationAsExcelFloat::from(value.mount.end_time - value.mount.start_time),
                ))),
            )))),
            ispyb_status: ActiveValue::Set(Some(NullAsEmptyString::from(NullAsLiteralNone::from(
                NullAsLiteralNa::from(ISPyBExportAsText::from(value.ispyb_export)),
            )))),
            data_collection_visit: ActiveValue::Set(Some(NullAsEmptyString::from(
                NullAsLiteralNone::from(NullAsLiteralNa::from(VisitAsText::from(
                    value.collection_visit,
                ))),
            ))),
            soak_db_comments: ActiveValue::Set(Some(value.comments)),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct WellReadback {
    id: i32,
    lab_visit_name: Option<Visit>,
    collection_visit_name: Option<Visit>,
    batch: Option<i32>,
    crystal: CrystalReadback,
    solvent: SolventReadback,
    cryo: CryoReadback,
    mount: MountReadback,
    ispyb_export: Option<ISPyBExport>,
    comments: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct CrystalReadback {
    plate: Option<String>,
    well: Option<String>,
    name: Option<String>,
    position: Option<Position>,
    drop_volume: Option<f64>,
    protein_name: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct SolventReadback {
    plate: Option<String>,
    well: Option<String>,
    name: Option<String>,
    smiles: Option<String>,
    code: Option<String>,
    stock_concentration: Option<f64>,
    concentration: Option<f64>,
    fraction: Option<f64>,
    transfer_volume: Option<f64>,
    status: Option<Status>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct CryoReadback {
    well: Option<String>,
    stock_fraction: Option<i32>,
    fraction: Option<i32>,
    transfer_volume: Option<f64>,
    status: Option<Status>,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "graphql-models", derive(SimpleObject))]
pub struct MountReadback {
    puck_barcode: Option<String>,
    puck_well: Option<i32>,
    pin_barcode: Option<String>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    harvest_status: Option<Status>,
    result: Option<MountingResult>,
}

impl From<crate::tables::main_table::Model> for WellReadback {
    fn from(value: crate::tables::main_table::Model) -> Self {
        let crystal_position = match (value.echo_x, value.echo_y) {
            (Some(x), Some(y)) => Some(Position {
                x: *****x,
                y: *****y,
            }),
            _ => None,
        };
        Self {
            id: value.id,
            lab_visit_name: value.lab_visit.map(|val| Visit::from((***val).clone())),
            collection_visit_name: value
                .data_collection_visit
                .map(|val| Visit::from((***val).clone())),
            batch: value.batch_number.map(|val| ****val),
            crystal: CrystalReadback {
                plate: value.crystal_plate,
                well: value.crystal_well,
                name: value.crystal_name,
                position: crystal_position,
                drop_volume: value.drop_volume.map(|val| *****val),
                protein_name: value.protein_name,
            },
            solvent: SolventReadback {
                plate: value.library_plate,
                well: value.source_well,
                name: value.library_name,
                smiles: value.compound_smiles,
                code: value.compound_code,
                stock_concentration: value.compound_stock_concentration.map(|val| *****val),
                concentration: value.compound_concentration.map(|val| *****val),
                fraction: value.solvent_fraction.map(|val| *****val),
                transfer_volume: value.soak_transfer_vol.map(|val| *****val),
                status: value.soak_status.map(|val| Status::from(***val)),
                timestamp: value.soak_timestamp.map(|val| *****val),
            },
            cryo: CryoReadback {
                well: value.cryo_well,
                stock_fraction: value.cryo_stock_fraction.map(|val| ****val),
                fraction: value.cryo_fraction.map(|val| ****val),
                transfer_volume: value.cryo_transfer_volume.map(|val| *****val),
                status: value.cryo_status.map(|val| Status::from(***val)),
                timestamp: value.cryo_timestamp.map(|val| *****val),
            },
            mount: MountReadback {
                puck_barcode: value.puck,
                puck_well: value.puck_position.map(|val| ****val),
                pin_barcode: value.pin_barcode,
                start_time: value.mounting_arrival_time.map(|val| *****val),
                end_time: value.mounted_timestamp.map(|val| *****val),
                harvest_status: value.harvest_status.map(|val| Status::from(***val)),
                result: value
                    .mounting_result
                    .map(|val| MountingResult::from((***val).clone())),
            },
            ispyb_export: value
                .ispyb_status
                .map(|val| ISPyBExport::from((***val).clone())),
            comments: value.soak_db_comments,
        }
    }
}
