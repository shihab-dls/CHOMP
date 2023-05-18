use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VisitAsText {
    pub(crate) proposal_type: [char; 2],
    pub(crate) proposal_number: u32,
    pub(crate) visit_number: u32,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum VisitParsingError {
    #[error("Visit text did not match the expected format.")]
    UnexpectedFormat,
    #[error("Proposal type code could not be parsed.")]
    UnparseableProposalType,
    #[error("Proposal number text could not be parsed.")]
    UnparseableProposalNumber(lexical::Error),
    #[error("Visit number text could not be parsed.")]
    UnparseableVisitNumber(lexical::Error),
}

impl FromStr for VisitAsText {
    type Err = VisitParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (proposal_text, visit_text) = s
            .split_once('-')
            .ok_or(VisitParsingError::UnexpectedFormat)?;
        let mut proposal_chars = proposal_text.chars();
        let proposal_type = [
            proposal_chars
                .next()
                .ok_or(VisitParsingError::UnparseableProposalType)?,
            proposal_chars
                .next()
                .ok_or(VisitParsingError::UnparseableProposalType)?,
        ];
        let proposal_number_text = proposal_chars.collect::<String>();
        let proposal_number = lexical::parse(proposal_number_text)
            .map_err(VisitParsingError::UnparseableProposalNumber)?;
        let visit_number =
            lexical::parse(visit_text).map_err(VisitParsingError::UnparseableVisitNumber)?;
        Ok(Self {
            proposal_type,
            proposal_number,
            visit_number,
        })
    }
}

impl ToString for VisitAsText {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}-{}",
            self.proposal_type[0], self.proposal_type[1], self.proposal_number, self.visit_number
        )
    }
}

impl From<VisitAsText> for Value {
    fn from(value: VisitAsText) -> Self {
        Value::String(Some(Box::new(value.to_string())))
    }
}

impl TryGetable for VisitAsText {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        Self::from_str(&text).map_err(|err| {
            TryGetError::DbErr(DbErr::Type(format!(
                "Could not parse '{}' as Visit for {:?}, got {}",
                text, index, err
            )))
        })
    }
}

impl ValueType for VisitAsText {
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

impl Nullable for VisitAsText {
    fn null() -> Value {
        String::null()
    }
}

#[cfg(test)]
mod tests {
    use crate::datatypes::visit::VisitAsText;
    use std::str::FromStr;

    #[test]
    fn visit_as_text_serializes() {
        assert_eq!(
            "cm32485-1",
            VisitAsText {
                proposal_type: ['c', 'm'],
                proposal_number: 32485,
                visit_number: 1
            }
            .to_string()
        )
    }

    #[test]
    fn visit_as_text_deserializes() {
        assert_eq!(
            VisitAsText {
                proposal_type: ['c', 'm'],
                proposal_number: 32485,
                visit_number: 1
            },
            VisitAsText::from_str("cm32485-1").unwrap()
        )
    }
}
