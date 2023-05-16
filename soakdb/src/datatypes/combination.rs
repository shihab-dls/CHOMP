use derive_more::{Deref, DerefMut, From};
use sea_orm::{
    sea_query::{ArrayType, Nullable, ValueType, ValueTypeErr},
    ColIdx, ColumnType, QueryResult, TryGetError, TryGetable, Value,
};
use std::{any::type_name, marker::PhantomData, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct AsSelfOr<RW, R>(
    #[deref]
    #[deref_mut]
    RW,
    PhantomData<R>,
)
where
    Value: From<RW>,
    RW: TryGetable + ValueType,
    R: TryGetable + ValueType + Into<RW>;

impl<RW, R> From<RW> for AsSelfOr<RW, R>
where
    Value: From<RW>,
    RW: TryGetable + ValueType,
    R: TryGetable + ValueType + Into<RW>,
{
    fn from(value: RW) -> Self {
        Self(value, PhantomData)
    }
}

impl<RW, R> From<AsSelfOr<RW, R>> for Value
where
    Value: From<RW>,
    Value: From<R>,
    RW: TryGetable + ValueType,
    R: TryGetable + ValueType + Into<RW>,
{
    fn from(value: AsSelfOr<RW, R>) -> Self {
        Value::from(value.0)
    }
}

impl<RW, R> FromStr for AsSelfOr<RW, R>
where
    Value: From<RW>,
    RW: TryGetable + ValueType + FromStr,
    R: TryGetable + ValueType + Into<RW> + FromStr,
{
    type Err = <RW as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_primary = s.parse::<RW>();
        let as_secondary = s.parse::<R>();
        match (as_primary, as_secondary) {
            (Ok(val), _) => Ok(Self(val, PhantomData)),
            (_, Ok(val)) => Ok(Self(val.into(), PhantomData)),
            (Err(err), Err(_)) => Err(err),
        }
    }
}

impl<RW, R> TryGetable for AsSelfOr<RW, R>
where
    Value: From<RW>,
    RW: TryGetable + ValueType,
    R: TryGetable + ValueType + Into<RW>,
{
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let as_primary = RW::try_get_by(res, index);
        let as_secondary = R::try_get_by(res, index);
        match (as_primary, as_secondary) {
            (Ok(val), _) => Ok(Self(val, PhantomData)),
            (_, Ok(val)) => Ok(Self(val.into(), PhantomData)),
            (Err(err), Err(_)) => Err(err),
        }
    }
}

impl<RW, R> ValueType for AsSelfOr<RW, R>
where
    Value: From<RW>,
    RW: TryGetable + ValueType,
    R: TryGetable + ValueType + Into<RW>,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        if let Ok(primary) = RW::try_from(v.clone()) {
            Ok(Self(primary, PhantomData))
        } else if let Ok(secondary) = R::try_from(v) {
            Ok(Self(secondary.into(), PhantomData))
        } else {
            Err(ValueTypeErr)
        }
    }

    fn type_name() -> String {
        type_name::<Self>().to_string()
    }

    fn array_type() -> ArrayType {
        RW::array_type()
    }

    fn column_type() -> ColumnType {
        RW::column_type()
    }
}

impl<RW, R> Nullable for AsSelfOr<RW, R>
where
    Value: From<RW>,
    RW: TryGetable + ValueType + Nullable,
    R: TryGetable + ValueType + Into<RW>,
{
    fn null() -> Value {
        RW::null()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deref, DerefMut, From)]
pub struct NeverRead<T>(T)
where
    Value: From<T>,
    T: ValueType;

impl<T> From<NeverRead<T>> for Value
where
    Value: From<T>,
    T: ValueType,
{
    fn from(value: NeverRead<T>) -> Self {
        Value::from(value.0)
    }
}

impl<T> FromStr for NeverRead<T>
where
    Value: From<T>,
    T: ValueType + FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<T>()?))
    }
}

impl<T> TryGetable for NeverRead<T>
where
    Value: From<T>,
    T: ValueType,
{
    fn try_get_by<I: ColIdx>(_res: &QueryResult, _index: I) -> Result<Self, TryGetError> {
        Err(TryGetError::Null(type_name::<I>().to_string()))
    }
}

impl<T> ValueType for NeverRead<T>
where
    Value: From<T>,
    T: ValueType,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        Ok(Self(T::try_from(v)?))
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

impl<T> Nullable for NeverRead<T>
where
    Value: From<T>,
    T: ValueType + Nullable,
{
    fn null() -> Value {
        T::null()
    }
}
