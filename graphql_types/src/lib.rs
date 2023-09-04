use async_graphql::{InputObject, SimpleObject};
use std::{num::ParseIntError, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone, SimpleObject, InputObject)]
#[graphql(input_name = "WellInput")]
pub struct Well {
    pub plate: Uuid,
    pub well: i16,
}

impl ToString for Well {
    fn to_string(&self) -> String {
        format!("{}/{}", self.plate, self.well)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Could not decode Well from str")]
pub enum WellFromStrError {
    #[error("Failed to split string by '/'")]
    Spliting,
    #[error("Could not parse prefix as Uuid")]
    PlateParsing(#[from] uuid::Error),
    #[error("Could not parse suffix as i16")]
    WellParsing(#[from] ParseIntError),
}

impl FromStr for Well {
    type Err = WellFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (plate, well) = s.split_once('/').ok_or(WellFromStrError::Spliting)?;

        Ok(Self {
            plate: plate.parse()?,
            well: well.parse()?,
        })
    }
}
