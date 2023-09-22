use std::{any::type_name, str::FromStr};

use derive_more::From;
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
pub enum StatusAsText {
    Success,
    Failure,
    Pending,
}

impl FromStr for StatusAsText {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "done" => Ok(Self::Success),
            "exported" => Ok(Self::Success),
            "fail" => Ok(Self::Failure),
            "pending" => Ok(Self::Pending),
            _ => Err(()),
        }
    }
}

impl ToString for StatusAsText {
    fn to_string(&self) -> String {
        match self {
            StatusAsText::Success => "done".to_string(),
            StatusAsText::Failure => "fail".to_string(),
            StatusAsText::Pending => "pending".to_string(),
        }
    }
}

impl From<StatusAsText> for Value {
    fn from(value: StatusAsText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for StatusAsText {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        Self::from_str(&text).map_err(|_| {
            TryGetError::DbErr(DbErr::Type(format!(
                "Could not parse '{}' as Status for {:?}",
                text, index
            )))
        })
    }
}

impl ValueType for StatusAsText {
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

impl Nullable for StatusAsText {
    fn null() -> Value {
        String::null()
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::status::StatusAsText;
    use std::str::FromStr;

    #[test]
    fn harvest_status_deserializes() {
        assert_eq!(
            StatusAsText::Success,
            StatusAsText::from_str("done").unwrap()
        );
        assert_eq!(
            StatusAsText::Success,
            StatusAsText::from_str("exported").unwrap()
        );
        assert_eq!(
            StatusAsText::Failure,
            StatusAsText::from_str("fail").unwrap()
        );
        assert_eq!(
            StatusAsText::Pending,
            StatusAsText::from_str("pending").unwrap()
        );
    }
}
