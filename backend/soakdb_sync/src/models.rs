use async_graphql::{Enum, ErrorExtensions, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use soakdb_io::Fallible;
use std::any::type_name;

#[derive(Debug, Clone, SimpleObject)]
pub struct SoakDBReadback {
    pub metadata: MetadataReadback,
    pub wells: Vec<WellReadback>,
}

#[derive(Debug, Clone, InputObject)]
pub struct Metadata {
    pub name: String,
    pub protein: String,
}

impl From<Metadata> for soakdb_io::Metadata {
    fn from(value: Metadata) -> Self {
        Self {
            name: value.name,
            protein: value.protein,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct MetadataReadback {
    pub name: Option<String>,
    pub protein: Option<String>,
}

impl From<soakdb_io::MetadataReadback> for MetadataReadback {
    fn from(value: soakdb_io::MetadataReadback) -> Self {
        Self {
            name: value.name,
            protein: value.protein,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
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

#[derive(Debug, Clone, InputObject)]
pub struct Visit {
    proposal_type: [char; 2],
    proposal_number: u32,
    visit_number: u32,
}

impl From<Visit> for soakdb_io::Visit {
    fn from(value: Visit) -> Self {
        Self {
            proposal_number: value.proposal_number,
            proposal_type: value.proposal_type,
            visit_number: value.visit_number,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct Crystal {
    pub plate: String,
    pub well: String,
    pub name: String,
    pub position: Position,
    pub drop_volume: f64,
    pub protein_name: String,
}

impl From<Crystal> for soakdb_io::Crystal {
    fn from(value: Crystal) -> Self {
        Self {
            plate: value.plate,
            well: value.well,
            name: value.name,
            position: value.position.into(),
            drop_volume: value.drop_volume,
            protein_name: value.protein_name,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl From<Position> for soakdb_io::Position {
    fn from(value: Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
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

impl From<Solvent> for soakdb_io::Solvent {
    fn from(value: Solvent) -> Self {
        Self {
            plate: value.plate,
            well: value.well,
            name: value.name,
            smiles: value.smiles,
            code: value.code,
            stock_concentration: value.stock_concentration,
            concentration: value.concentration,
            fraction: value.fraction,
            transfer_volume: value.transfer_volume,
            status: value.status.into(),
            timestamp: value.timestamp,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct Cryo {
    pub well: String,
    pub stock_fraction: i32,
    pub fraction: i32,
    pub transfer_volume: f64,
    pub status: Status,
    pub timestamp: DateTime<Utc>,
}

impl From<Cryo> for soakdb_io::Cryo {
    fn from(value: Cryo) -> Self {
        Self {
            well: value.well,
            stock_fraction: value.stock_fraction,
            fraction: value.fraction,
            transfer_volume: value.transfer_volume,
            status: value.status.into(),
            timestamp: value.timestamp,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct Mount {
    pub puck_barcode: String,
    pub puck_well: i32,
    pub pin_barcode: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub harvest_status: Status,
    pub result: MountingResult,
}

impl From<Mount> for soakdb_io::Mount {
    fn from(value: Mount) -> Self {
        Self {
            puck_barcode: value.puck_barcode,
            puck_well: value.puck_well,
            pin_barcode: value.pin_barcode,
            start_time: value.start_time,
            end_time: value.end_time,
            harvest_status: value.harvest_status.into(),
            result: value.result.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

impl From<Status> for soakdb_io::Status {
    fn from(value: Status) -> Self {
        match value {
            Status::Success => Self::Success,
            Status::Failure => Self::Failure,
            Status::Pending => Self::Pending,
        }
    }
}

impl From<soakdb_io::Status> for Status {
    fn from(value: soakdb_io::Status) -> Self {
        match value {
            soakdb_io::Status::Success => Self::Success,
            soakdb_io::Status::Failure => Self::Failure,
            soakdb_io::Status::Pending => Self::Pending,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct MountingResult {
    pub success: bool,
    pub comment_1: String,
    pub comment_2: String,
}

impl From<MountingResult> for soakdb_io::MountingResult {
    fn from(value: MountingResult) -> Self {
        Self {
            success: value.success,
            comment_1: value.comment_1,
            comment_2: value.comment_2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum ISPyBExport {
    Exported,
    Pending,
}

impl From<ISPyBExport> for soakdb_io::ISPyBExport {
    fn from(value: ISPyBExport) -> Self {
        match value {
            ISPyBExport::Exported => Self::Exported,
            ISPyBExport::Pending => Self::Pending,
        }
    }
}

impl From<soakdb_io::ISPyBExport> for ISPyBExport {
    fn from(value: soakdb_io::ISPyBExport) -> Self {
        match value {
            soakdb_io::ISPyBExport::Exported => Self::Exported,
            soakdb_io::ISPyBExport::Pending => Self::Pending,
        }
    }
}

impl From<Well> for soakdb_io::Well {
    fn from(value: Well) -> Self {
        Self {
            lab_visit: value.lab_visit.into(),
            collection_visit: value.collection_visit.into(),
            batch: value.batch,
            crystal: value.crystal.into(),
            solvent: value.solvent.into(),
            cryo: value.cryo.into(),
            mount: value.mount.into(),
            ispyb_export: value.ispyb_export.into(),
            comments: value.comments,
        }
    }
}

fn into_parsing_error<T>(value: Fallible<T>) -> async_graphql::Result<T> {
    Result::from(value).map_err(|value| {
        async_graphql::Error::new(format!(
            "Could not parse '{}' into '{}'",
            value,
            type_name::<T>()
        ))
        .extend_with(|_, e| e.set("value", value))
        .extend_with(|_, e| e.set("type", type_name::<T>()))
    })
}

#[derive(Debug, Clone, SimpleObject)]
pub struct WellReadback {
    pub id: i32,
    pub lab_visit_name: async_graphql::Result<Option<VisitReadback>>,
    pub collection_visit_name: async_graphql::Result<Option<VisitReadback>>,
    pub batch: async_graphql::Result<Option<i32>>,
    pub crystal: CrystalReadback,
    pub solvent: SolventReadback,
    pub cryo: CryoReadback,
    pub mount: MountReadback,
    pub ispyb_export: async_graphql::Result<Option<ISPyBExport>>,
    pub comments: async_graphql::Result<Option<String>>,
}

impl From<soakdb_io::WellReadback> for WellReadback {
    fn from(value: soakdb_io::WellReadback) -> Self {
        Self {
            id: value.id,
            lab_visit_name: into_parsing_error(value.lab_visit_name.map_opt(VisitReadback::from)),
            collection_visit_name: into_parsing_error(
                value.collection_visit_name.map_opt(VisitReadback::from),
            ),
            batch: into_parsing_error(value.batch),
            crystal: value.crystal.into(),
            solvent: value.solvent.into(),
            cryo: value.cryo.into(),
            mount: value.mount.into(),
            ispyb_export: into_parsing_error(value.ispyb_export.map_opt(ISPyBExport::from)),
            comments: into_parsing_error(value.comments),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct VisitReadback {
    proposal_type: [char; 2],
    proposal_number: u32,
    visit_number: u32,
}

impl From<soakdb_io::Visit> for VisitReadback {
    fn from(value: soakdb_io::Visit) -> Self {
        Self {
            proposal_type: value.proposal_type,
            proposal_number: value.proposal_number,
            visit_number: value.visit_number,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct CrystalReadback {
    pub plate: async_graphql::Result<Option<String>>,
    pub well: async_graphql::Result<Option<String>>,
    pub name: async_graphql::Result<Option<String>>,
    pub position: async_graphql::Result<Option<PositionReadback>>,
    pub drop_volume: async_graphql::Result<Option<f64>>,
    pub protein_name: async_graphql::Result<Option<String>>,
}

impl From<soakdb_io::CrystalReadback> for CrystalReadback {
    fn from(value: soakdb_io::CrystalReadback) -> Self {
        Self {
            plate: into_parsing_error(value.plate),
            well: into_parsing_error(value.well),
            name: into_parsing_error(value.name),
            position: into_parsing_error(value.position.map_opt(PositionReadback::from)),
            drop_volume: into_parsing_error(value.drop_volume),
            protein_name: into_parsing_error(value.protein_name),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct PositionReadback {
    pub x: f64,
    pub y: f64,
}

impl From<soakdb_io::Position> for PositionReadback {
    fn from(value: soakdb_io::Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct SolventReadback {
    pub plate: async_graphql::Result<Option<String>>,
    pub well: async_graphql::Result<Option<String>>,
    pub name: async_graphql::Result<Option<String>>,
    pub smiles: async_graphql::Result<Option<String>>,
    pub code: async_graphql::Result<Option<String>>,
    pub stock_concentration: async_graphql::Result<Option<f64>>,
    pub concentration: async_graphql::Result<Option<f64>>,
    pub fraction: async_graphql::Result<Option<f64>>,
    pub transfer_volume: async_graphql::Result<Option<f64>>,
    pub status: async_graphql::Result<Option<Status>>,
    pub timestamp: async_graphql::Result<Option<DateTime<Utc>>>,
}

impl From<soakdb_io::SolventReadback> for SolventReadback {
    fn from(value: soakdb_io::SolventReadback) -> Self {
        Self {
            plate: into_parsing_error(value.plate),
            well: into_parsing_error(value.well),
            name: into_parsing_error(value.name),
            smiles: into_parsing_error(value.smiles),
            code: into_parsing_error(value.code),
            stock_concentration: into_parsing_error(value.stock_concentration),
            concentration: into_parsing_error(value.concentration),
            fraction: into_parsing_error(value.fraction),
            transfer_volume: into_parsing_error(value.transfer_volume),
            status: into_parsing_error(value.status.map_opt(Status::from)),
            timestamp: into_parsing_error(value.timestamp),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct CryoReadback {
    pub well: async_graphql::Result<Option<String>>,
    pub stock_fraction: async_graphql::Result<Option<i32>>,
    pub fraction: async_graphql::Result<Option<i32>>,
    pub transfer_volume: async_graphql::Result<Option<f64>>,
    pub status: async_graphql::Result<Option<Status>>,
    pub timestamp: async_graphql::Result<Option<DateTime<Utc>>>,
}

impl From<soakdb_io::CryoReadback> for CryoReadback {
    fn from(value: soakdb_io::CryoReadback) -> Self {
        Self {
            well: into_parsing_error(value.well),
            stock_fraction: into_parsing_error(value.stock_fraction),
            fraction: into_parsing_error(value.fraction),
            transfer_volume: into_parsing_error(value.transfer_volume),
            status: into_parsing_error(value.status.map_opt(Status::from)),
            timestamp: into_parsing_error(value.timestamp),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct MountReadback {
    pub puck_barcode: async_graphql::Result<Option<String>>,
    pub puck_well: async_graphql::Result<Option<i32>>,
    pub pin_barcode: async_graphql::Result<Option<String>>,
    pub start_time: async_graphql::Result<Option<DateTime<Utc>>>,
    pub end_time: async_graphql::Result<Option<DateTime<Utc>>>,
    pub harvest_status: async_graphql::Result<Option<Status>>,
    pub result: async_graphql::Result<Option<MountingResultReadback>>,
}

impl From<soakdb_io::MountReadback> for MountReadback {
    fn from(value: soakdb_io::MountReadback) -> Self {
        Self {
            puck_barcode: into_parsing_error(value.puck_barcode),
            puck_well: into_parsing_error(value.puck_well),
            pin_barcode: into_parsing_error(value.pin_barcode),
            start_time: into_parsing_error(value.start_time),
            end_time: into_parsing_error(value.end_time),
            harvest_status: into_parsing_error(value.harvest_status.map_opt(Status::from)),
            result: into_parsing_error(value.result.map_opt(MountingResultReadback::from)),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct MountingResultReadback {
    pub success: bool,
    pub comment_1: String,
    pub comment_2: String,
}

impl From<soakdb_io::MountingResult> for MountingResultReadback {
    fn from(value: soakdb_io::MountingResult) -> Self {
        Self {
            success: value.success,
            comment_1: value.comment_1,
            comment_2: value.comment_2,
        }
    }
}
