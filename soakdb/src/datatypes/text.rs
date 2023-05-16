use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, num::NonZeroI32, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct AsSelfOrText<T>(T)
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable;

impl<T> FromStr for AsSelfOrText<T>
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable,
{
    type Err = <T as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl<T> From<AsSelfOrText<T>> for Value
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable,
{
    fn from(value: AsSelfOrText<T>) -> Self {
        value.0.into()
    }
}

impl<T> TryGetable for AsSelfOrText<T>
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let as_self = T::try_get_by(res, index);
        let as_text = String::try_get_by(res, index).map(|text| text.trim().to_string());
        match (as_self, as_text) {
            (Ok(val), _) => Ok(Self(val)),
            (_, Ok(text)) => {
                if text.is_empty() {
                    Err(TryGetError::Null(index.as_str().unwrap().to_string()))
                } else {
                    Ok(Self(text.parse().map_err(|_| {
                        TryGetError::DbErr(DbErr::Type(format!(
                            "Could not parse '{}' into {} for {:?}",
                            text,
                            type_name::<T>(),
                            index
                        )))
                    })?))
                }
            }
            (Err(TryGetError::Null(err)), _) => Err(TryGetError::Null(err)),
            (_, Err(TryGetError::Null(err))) => Err(TryGetError::Null(err)),
            (Err(TryGetError::DbErr(self_err)), Err(TryGetError::DbErr(text_err))) => {
                Err(TryGetError::DbErr(DbErr::Type(format!(
                    "Could not retrieve value as {} or String for {:?}. Got {} and {}",
                    type_name::<T>(),
                    index,
                    self_err,
                    text_err
                ))))
            }
        }
    }
}

impl<T> ValueType for AsSelfOrText<T>
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if let Ok(normal) = T::try_from(v.clone()) {
            Ok(Self(normal))
        } else if let Value::String(Some(text)) = v {
            text.parse()
                .map(|value| Self(value))
                .map_err(|_| ValueTypeErr)
        } else {
            Err(ValueTypeErr)
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}

impl<T> Nullable for AsSelfOrText<T>
where
    T: Copy + FromStr + Into<Value> + TryGetable + ValueType + Nullable,
{
    fn null() -> Value {
        T::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct NullAsLiteralNone<T>(T)
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable;

impl<T> From<NullAsLiteralNone<T>> for Value
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable,
{
    fn from(value: NullAsLiteralNone<T>) -> Self {
        Value::from(value.0)
    }
}

impl<T> TryGetable for NullAsLiteralNone<T>
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let as_self = T::try_get_by(res, index);
        let as_literal_none = String::try_get_by(res, index).map(|text| {
            if text == "None" {
                TryGetError::Null(index.as_str().unwrap().to_string())
            } else {
                TryGetError::DbErr(DbErr::Type(format!(
                    "Retrieved text ({}) was not literal 'None'.",
                    text
                )))
            }
        });
        match (as_self, as_literal_none) {
            (Ok(val), _) => Ok(Self(val)),
            (_, Ok(TryGetError::Null(val))) => Err(TryGetError::Null(val)),
            (Err(err), _) => Err(err),
        }
    }
}

impl<T> ValueType for NullAsLiteralNone<T>
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        T::try_from(v).map(|value| Self(value))
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> ArrayType {
        T::array_type()
    }

    fn column_type() -> ColumnType {
        T::column_type()
    }
}

impl<T> Nullable for NullAsLiteralNone<T>
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable,
{
    fn null() -> Value {
        T::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct FloatAsScientificText<T>(T)
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType;

impl<T> ToString for FloatAsScientificText<T>
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType,
{
    fn to_string(&self) -> String {
        let options = lexical::WriteFloatOptions::builder()
            .negative_exponent_break(NonZeroI32::new(-3))
            .exponent(b'E')
            .build()
            .unwrap();
        let value = f32::from(self.0);
        lexical::to_string_with_options::<_, { lexical::format::STANDARD }>(value, &options)
    }
}

impl<T> FromStr for FloatAsScientificText<T>
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType,
{
    type Err = lexical::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(lexical::parse::<f32, _>(s)?.into()))
    }
}

impl<T> From<FloatAsScientificText<T>> for Value
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType,
{
    fn from(value: FloatAsScientificText<T>) -> Self {
        Value::String(Some(Box::new(value.to_string())))
    }
}

impl<T> TryGetable for FloatAsScientificText<T>
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let text = String::try_get_by(res, index)?.trim().to_string();
        if text.is_empty() {
            return Err(TryGetError::Null(type_name::<Self>().to_string()));
        }
        Ok(Self::from_str(&String::try_get_by(res, index)?)
            .map_err(|err| DbErr::Type(format!("Failed to parse {:?} with: {}", index, err)))?)
    }
}

impl<T> ValueType for FloatAsScientificText<T>
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        T::try_from(v).map(|value| Self(value))
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

impl<T> Nullable for FloatAsScientificText<T>
where
    f32: From<T>,
    T: From<f32> + Copy + ValueType + Nullable,
{
    fn null() -> Value {
        T::null()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::datatypes::text::FloatAsScientificText;

    #[test]
    fn float_as_scientific_notation_serializes() {
        assert_eq!("0.00123", FloatAsScientificText(0.00123).to_string());
        assert_eq!("1.23E-4", FloatAsScientificText(0.000123).to_string());
    }

    #[test]
    fn float_as_scientific_notation_deserializes() {
        assert_eq!(
            0.00123,
            FloatAsScientificText::<f32>::from_str("0.00123").unwrap().0
        );
        assert_eq!(
            0.000123,
            FloatAsScientificText::<f32>::from_str("1.23E-4").unwrap().0
        );
    }
}
