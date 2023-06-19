use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ISPyBExportAsText {
    Exported,
    ExportedTo(PathBuf),
    Pending,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ISPyBExportParsingError {
    #[error("Could not determine export status")]
    UnknownStatus,
    #[error("Could not determine exported path")]
    UnknownExportPath,
}

impl FromStr for ISPyBExportAsText {
    type Err = ISPyBExportParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPORTED_TO_PREFIX: &str = "exported to ";
        if s.starts_with(EXPORTED_TO_PREFIX) {
            Ok(Self::ExportedTo(PathBuf::from(
                s.strip_prefix(EXPORTED_TO_PREFIX)
                    .ok_or(ISPyBExportParsingError::UnknownExportPath)?,
            )))
        } else if s.starts_with("exported") {
            Ok(Self::Exported)
        } else if s.starts_with("pending") {
            Ok(Self::Pending)
        } else {
            Err(ISPyBExportParsingError::UnknownStatus)
        }
    }
}

impl ToString for ISPyBExportAsText {
    fn to_string(&self) -> String {
        match self {
            Self::Exported => "exported".to_string(),
            Self::ExportedTo(path) => format!("exported to {}", path.to_str().unwrap()),
            Self::Pending => "pending".to_string(),
        }
    }
}

impl From<ISPyBExportAsText> for Value {
    fn from(value: ISPyBExportAsText) -> Self {
        Self::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for ISPyBExportAsText {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        Self::from_str(&text).map_err(|err| {
            TryGetError::DbErr(DbErr::Type(format!(
                "Could not parse '{}' as ISPyBExport for {:?}, got {}",
                text, index, err
            )))
        })
    }
}

impl ValueType for ISPyBExportAsText {
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
        String::array_type()
    }

    fn column_type() -> ColumnType {
        String::column_type()
    }
}

impl Nullable for ISPyBExportAsText {
    fn null() -> Value {
        String::null()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::datatypes::ispyb_export::ISPyBExportAsText;

    #[test]
    fn ispyb_export_as_text_serializes() {
        assert_eq!("exported", ISPyBExportAsText::Exported.to_string());
        assert_eq!(
            "exported to /my/file",
            ISPyBExportAsText::ExportedTo(PathBuf::from("/my/file")).to_string()
        );
        assert_eq!("pending", ISPyBExportAsText::Pending.to_string());
    }
    #[test]
    fn ispyb_export_as_text_deserializes() {
        assert_eq!(
            ISPyBExportAsText::Exported,
            ISPyBExportAsText::from_str("exported").unwrap()
        );
        assert_eq!(
            ISPyBExportAsText::ExportedTo(PathBuf::from("/my/file")),
            ISPyBExportAsText::from_str("exported to /my/file").unwrap()
        );
        assert_eq!(
            ISPyBExportAsText::Pending,
            ISPyBExportAsText::from_str("pending").unwrap()
        );
    }
}
