use chrono::{Duration, NaiveTime};
use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{Nullable, ValueType},
    TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

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
        if text.is_empty() {
            Err(sea_orm::TryGetError::Null(type_name::<I>().to_string()))
        } else {
            text.parse().map_err(|err| {
                sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(format!(
                    "Could not parse '{}' as Duration using format '{}' for {:?}: {}",
                    text, DURATION_FORMAT, index, err
                )))
            })
        }
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

#[cfg(test)]
mod tests {
    use super::DurationAsText;
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
}
