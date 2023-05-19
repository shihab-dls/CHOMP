use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    strum::Display,
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MountingResultAsText {
    pub success: bool,
    pub comment_1: String,
    pub comment_2: String,
}

#[derive(Debug, Display, Copy, Clone, thiserror::Error)]
pub enum MountingResultParsingError {
    UnknownStatus,
    MissingField,
    ExtraField,
}

impl FromStr for MountingResultAsText {
    type Err = MountingResultParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(':');
        let success_text = components
            .next()
            .ok_or(MountingResultParsingError::MissingField)?
            .trim()
            .to_string();
        let comment_1 = components
            .next()
            .ok_or(MountingResultParsingError::MissingField)?
            .trim()
            .to_string();
        let comment_2 = components
            .next()
            .ok_or(MountingResultParsingError::MissingField)?
            .trim()
            .to_string();
        if components.next().is_some() {
            return Err(MountingResultParsingError::ExtraField);
        }

        let success = match success_text.as_str() {
            "OK" => Ok(true),
            "FAIL" => Ok(false),
            _ => Err(MountingResultParsingError::UnknownStatus),
        }?;

        Ok(Self {
            success,
            comment_1,
            comment_2,
        })
    }
}

impl ToString for MountingResultAsText {
    fn to_string(&self) -> String {
        let success_text = if self.success { "OK" } else { "FAIL" };
        format!("{}: {}: {}", success_text, self.comment_1, self.comment_2)
    }
}

impl From<MountingResultAsText> for Value {
    fn from(value: MountingResultAsText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for MountingResultAsText {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        Self::from_str(&text).map_err(|_| {
            TryGetError::DbErr(DbErr::Type(format!(
                "Could not parse '{}' as MountingResult for {:?}",
                text, index
            )))
        })
    }
}

impl ValueType for MountingResultAsText {
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

impl Nullable for MountingResultAsText {
    fn null() -> Value {
        String::null()
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::mounting_result::MountingResultAsText;
    use std::str::FromStr;

    #[test]
    fn mounting_result_as_text_serializes() {
        assert_eq!(
            "OK: One: Two",
            MountingResultAsText {
                success: true,
                comment_1: "One".to_string(),
                comment_2: "Two".to_string()
            }
            .to_string()
        );
        assert_eq!(
            "FAIL: One: Two",
            MountingResultAsText {
                success: false,
                comment_1: "One".to_string(),
                comment_2: "Two".to_string()
            }
            .to_string()
        );
    }

    #[test]
    fn mounting_result_as_text_deserializes() {
        assert_eq!(
            MountingResultAsText {
                success: true,
                comment_1: "One".to_string(),
                comment_2: "Two".to_string()
            },
            MountingResultAsText::from_str("OK: One:Two").unwrap()
        );
        assert_eq!(
            MountingResultAsText {
                success: false,
                comment_1: "One".to_string(),
                comment_2: "Two".to_string()
            },
            MountingResultAsText::from_str("FAIL: One:Two").unwrap()
        );
    }
}
