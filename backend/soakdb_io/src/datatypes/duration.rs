use super::combination::AsSelfOr;
use chrono::{Duration, NaiveTime};
use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColumnType, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

pub type DurationAsVarious = AsSelfOr<DurationAsText, DurationAsTimeText>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DurationAsText(Duration);

static DURATION_FORMAT: &str = "%H:%M:%S";

impl FromStr for DurationAsText {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveTime::parse_from_str(s, DURATION_FORMAT).map(|time| Self(time - NaiveTime::MIN))
    }
}

impl ToString for DurationAsText {
    fn to_string(&self) -> String {
        (NaiveTime::MIN + self.0)
            .format(DURATION_FORMAT)
            .to_string()
    }
}

impl From<DurationAsText> for Value {
    fn from(value: DurationAsText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for DurationAsText {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        text.parse().map_err(|err| {
            sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(format!(
                "Could not parse '{}' as Duration using format '{}' for {:?}: {}",
                text, DURATION_FORMAT, index, err
            )))
        })
    }
}

impl ValueType for DurationAsText {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            Value::String(Some(val)) => val.parse().map_err(|_| sea_orm::sea_query::ValueTypeErr),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::ColumnType {
        sea_orm::ColumnType::String(None)
    }
}

impl Nullable for DurationAsText {
    fn null() -> Value {
        String::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DurationAsTimeText(Duration);

static DURATION_FORMAT_TIME: &str = "%H:%M:%S AM";

impl FromStr for DurationAsTimeText {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveTime::parse_from_str(s, DURATION_FORMAT_TIME).map(|time| Self(time - NaiveTime::MIN))
    }
}

impl ToString for DurationAsTimeText {
    fn to_string(&self) -> String {
        (NaiveTime::MIN + self.0)
            .format(DURATION_FORMAT_TIME)
            .to_string()
    }
}

impl From<DurationAsTimeText> for Value {
    fn from(value: DurationAsTimeText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for DurationAsTimeText {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        text.parse().map_err(|err| {
            sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(format!(
                "Could not parse '{}' as Duration using format '{}' for {:?}: {}",
                text, DURATION_FORMAT_TIME, index, err
            )))
        })
    }
}

impl ValueType for DurationAsTimeText {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            Value::String(Some(val)) => val.parse().map_err(|_| sea_orm::sea_query::ValueTypeErr),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::ColumnType {
        sea_orm::ColumnType::String(None)
    }
}

impl Nullable for DurationAsTimeText {
    fn null() -> Value {
        String::null()
    }
}

impl From<DurationAsTimeText> for DurationAsText {
    fn from(value: DurationAsTimeText) -> Self {
        Self(value.0)
    }
}

impl From<DurationAsText> for DurationAsTimeText {
    fn from(value: DurationAsText) -> Self {
        Self(value.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DurationAsExcelFloat(Duration);

impl From<f32> for DurationAsExcelFloat {
    fn from(value: f32) -> Self {
        let days_from_epoch = Duration::days(value.floor() as i64);
        let seconds_from_midnight =
            Duration::seconds((value.fract() * Duration::days(1).num_seconds() as f32) as i64);
        Self(days_from_epoch + seconds_from_midnight)
    }
}

impl From<DurationAsExcelFloat> for f32 {
    fn from(value: DurationAsExcelFloat) -> Self {
        let num_days = value.0.num_days();
        let num_seconds = (value.0 - Duration::days(num_days)).num_seconds();
        let proportion_of_day = num_seconds as f32 / Duration::days(1).num_seconds() as f32;
        num_days as f32 + proportion_of_day
    }
}

impl From<DurationAsExcelFloat> for Value {
    fn from(value: DurationAsExcelFloat) -> Self {
        Self::Float(Some(value.into()))
    }
}

impl TryGetable for DurationAsExcelFloat {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, TryGetError> {
        f32::try_get_by(res, index).map(Self::from)
    }
}

impl ValueType for DurationAsExcelFloat {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            Value::Float(Some(val)) => Ok(Self::from(val)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        ArrayType::Float
    }

    fn column_type() -> sea_orm::ColumnType {
        ColumnType::Float
    }
}

impl Nullable for DurationAsExcelFloat {
    fn null() -> Value {
        f32::null()
    }
}

impl From<DurationAsExcelFloat> for DurationAsText {
    fn from(value: DurationAsExcelFloat) -> Self {
        Self(value.0)
    }
}

impl From<DurationAsText> for DurationAsExcelFloat {
    fn from(value: DurationAsText) -> Self {
        Self(value.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::duration::DurationAsExcelFloat;

    use super::{DurationAsText, DurationAsTimeText};
    use chrono::Duration;
    use std::str::FromStr;

    fn test_duration() -> Duration {
        Duration::hours(3) + Duration::minutes(14) + Duration::seconds(15)
    }

    #[test]
    fn duration_as_text_serializes() {
        assert_eq!("03:14:15", DurationAsText(test_duration()).to_string())
    }

    #[test]
    fn duration_as_text_deserializes() {
        assert_eq!(
            test_duration(),
            DurationAsText::from_str("03:14:15").unwrap().0
        )
    }

    #[test]
    fn duration_as_time_text_serializes() {
        assert_eq!(
            "03:14:15 AM",
            DurationAsTimeText(test_duration()).to_string()
        )
    }

    #[test]
    fn duration_as_time_text_deserializes() {
        assert_eq!(
            test_duration(),
            DurationAsTimeText::from_str("03:14:15 AM").unwrap().0
        )
    }

    #[test]
    fn duration_as_excel_float_serializes() {
        assert_eq!(
            0.134_895_83,
            f32::from(DurationAsExcelFloat(test_duration()))
        )
    }

    #[test]
    fn duration_as_excel_float_deserializes() {
        assert_eq!(test_duration(), DurationAsExcelFloat::from(0.134_895_83).0)
    }
}
