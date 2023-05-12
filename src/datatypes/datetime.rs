use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct DateTimeAsEuroText(DateTime<Utc>);

static DATE_TIME_FORMAT: &str = "%d/%m/%Y %H:%M:%S";

impl FromStr for DateTimeAsEuroText {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            London
                .datetime_from_str(s, DATE_TIME_FORMAT)?
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
        if text.is_empty() {
            Err(TryGetError::Null(type_name::<I>().to_string()))
        } else {
            text.parse().map_err(|err| {
                TryGetError::DbErr(DbErr::Type(format!(
                    "Could not parse '{}' as DateTime using format '{}' for {:?}: {}",
                    text, DATE_TIME_FORMAT, index, err
                )))
            })
        }
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

#[cfg(test)]
mod tests {
    use super::DateTimeAsEuroText;
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
}
