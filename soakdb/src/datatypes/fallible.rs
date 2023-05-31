use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, fmt::Debug};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FallibleRead<T>
where
    Value: From<T>,
    T: TryGetable + ValueType,
{
    Ok(T),
    Fail(String),
}

impl<T> From<FallibleRead<T>> for Value
where
    Value: From<T>,
    T: TryGetable + ValueType,
{
    fn from(value: FallibleRead<T>) -> Self {
        match value {
            FallibleRead::Ok(val) => Self::from(val),
            FallibleRead::Fail(_) => {
                panic!("'FalliableRead::Fail' cannot be converted to 'Value'.")
            }
        }
    }
}

impl<T> TryGetable for FallibleRead<T>
where
    Value: From<T>,
    T: Debug + TryGetable + ValueType,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        match T::try_get_by(res, index) {
            Ok(val) => Ok(Self::Ok(val)),
            Err(_) => {
                let try_as_all = [
                    bool::try_get_by(res, index).map(|val| val.to_string()),
                    i8::try_get_by(res, index).map(|val| val.to_string()),
                    i16::try_get_by(res, index).map(|val| val.to_string()),
                    i32::try_get_by(res, index).map(|val| val.to_string()),
                    i64::try_get_by(res, index).map(|val| val.to_string()),
                    u8::try_get_by(res, index).map(|val| val.to_string()),
                    u16::try_get_by(res, index).map(|val| val.to_string()),
                    f32::try_get_by(res, index).map(|val| val.to_string()),
                    f64::try_get_by(res, index).map(|val| val.to_string()),
                    String::try_get_by(res, index),
                    Vec::<u8>::try_get_by(res, index)
                        .map(|val| val.into_iter().map(|byte| byte.to_string()).collect()),
                ];
                let found = try_as_all
                    .into_iter()
                    .find_map(Result::ok)
                    .unwrap_or("Unreadable Value".to_string());

                Ok(Self::Fail(found))
            }
        }
    }
}

impl<T> ValueType for FallibleRead<T>
where
    Value: From<T>,
    T: TryGetable + ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match T::try_from(v) {
            Ok(val) => Ok(Self::Ok(val)),
            Err(err) => Ok(Self::Fail(err.to_string())),
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

impl<T> Nullable for FallibleRead<T>
where
    Value: From<T>,
    T: TryGetable + ValueType + Nullable,
{
    fn null() -> Value {
        T::null()
    }
}
