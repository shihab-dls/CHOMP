use super::combination::AsSelfOr;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

pub type DateTimeAsVarious = AsSelfOr<DateTimeAsEuroText, DateTimeAsExcelFloat>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DateTimeAsEuroText(DateTime<Utc>);

static DATE_TIME_FORMAT: &str = "%d/%m/%Y %H:%M:%S";

#[derive(Debug, thiserror::Error)]
pub enum DateTimeAsEuroTextError {
    #[error("Could not parse date time string")]
    ParseError(#[from] chrono::ParseError),
    #[error("Parsed time is invalid")]
    TimeZoneError,
}

impl FromStr for DateTimeAsEuroText {
    type Err = DateTimeAsEuroTextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            NaiveDateTime::parse_from_str(s, DATE_TIME_FORMAT)?
                .and_local_timezone(London)
                .single()
                .ok_or(DateTimeAsEuroTextError::TimeZoneError)?
                .with_timezone(&Utc),
        ))
    }
}

impl ToString for DateTimeAsEuroText {
    fn to_string(&self) -> String {
        London
            .from_utc_datetime(&self.0.naive_utc())
            .format(DATE_TIME_FORMAT)
            .to_string()
    }
}

impl From<DateTimeAsEuroText> for Value {
    fn from(value: DateTimeAsEuroText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for DateTimeAsEuroText {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        text.parse().map_err(|err| {
            TryGetError::DbErr(DbErr::Type(format!(
                "Could not parse '{}' as DateTime using format '{}' for {:?}: {}",
                text, DATE_TIME_FORMAT, index, err
            )))
        })
    }
}

impl ValueType for DateTimeAsEuroText {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(val)) => val.parse().map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(None)
    }
}

impl Nullable for DateTimeAsEuroText {
    fn null() -> Value {
        String::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DateTimeAsExcelFloat(DateTime<Utc>);

impl DateTimeAsExcelFloat {
    fn epoch() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(1899, 12, 30)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    }
}

impl From<f32> for DateTimeAsExcelFloat {
    fn from(value: f32) -> Self {
        let days_from_epoch = Duration::days(value.floor() as i64);
        let seconds_from_midnight =
            Duration::seconds((value.fract() * Duration::days(1).num_seconds() as f32) as i64);
        let naive_date_time =
            DateTimeAsExcelFloat::epoch() + days_from_epoch + seconds_from_midnight;
        Self(
            naive_date_time
                .and_local_timezone(London)
                .unwrap()
                .with_timezone(&Utc),
        )
    }
}

impl From<DateTimeAsExcelFloat> for f32 {
    fn from(value: DateTimeAsExcelFloat) -> Self {
        let duration_from_lotus_epoch =
            value.0.with_timezone(&London).naive_local() - DateTimeAsExcelFloat::epoch();
        let num_days = duration_from_lotus_epoch.num_days();
        let num_seconds = (duration_from_lotus_epoch - Duration::days(num_days)).num_seconds();
        let proportion_of_day = num_seconds as f32 / Duration::days(1).num_seconds() as f32;
        num_days as f32 + proportion_of_day
    }
}

impl From<DateTimeAsExcelFloat> for Value {
    fn from(value: DateTimeAsExcelFloat) -> Self {
        Self::Float(Some(value.into()))
    }
}

impl TryGetable for DateTimeAsExcelFloat {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        f32::try_get_by(res, index).map(Self::from)
    }
}

impl ValueType for DateTimeAsExcelFloat {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Float(Some(val)) => Ok(Self::from(val)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> ArrayType {
        ArrayType::Float
    }

    fn column_type() -> ColumnType {
        ColumnType::Float
    }
}

impl Nullable for DateTimeAsExcelFloat {
    fn null() -> Value {
        f32::null()
    }
}

impl From<DateTimeAsExcelFloat> for DateTimeAsEuroText {
    fn from(value: DateTimeAsExcelFloat) -> Self {
        Self(value.0)
    }
}

impl From<DateTimeAsEuroText> for DateTimeAsExcelFloat {
    fn from(value: DateTimeAsEuroText) -> Self {
        Self(value.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{DateTimeAsEuroText, DateTimeAsExcelFloat};
    use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
    use std::str::FromStr;

    fn test_date_time() -> DateTime<Utc> {
        Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2023, 5, 11)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(13, 10, 37).unwrap()),
        )
    }

    #[test]
    fn date_time_as_euro_text_serializes() {
        assert_eq!(
            "11/05/2023 14:10:37",
            DateTimeAsEuroText(test_date_time()).to_string()
        )
    }

    #[test]
    fn date_time_as_euro_text_deserializes() {
        assert_eq!(
            test_date_time(),
            DateTimeAsEuroText::from_str("11/05/2023 14:10:37")
                .unwrap()
                .0
        )
    }

    #[test]
    fn date_time_as_excel_float_serializes() {
        assert_eq!(45_057.59, f32::from(DateTimeAsExcelFloat(test_date_time())))
    }

    /// Check f32 is deserialized to a time within a minute of the expected.
    #[test]
    fn date_time_as_excel_float_deserializes() {
        let deserialized_date_time = DateTimeAsExcelFloat::from(45_057.59).0;
        assert!(
            (deserialized_date_time - test_date_time())
                .num_minutes()
                .abs()
                <= 1
        )
    }
}
