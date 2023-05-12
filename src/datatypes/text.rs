use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, str::FromStr};

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
