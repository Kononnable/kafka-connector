use std::convert::TryInto;

use bytes::{Bytes, BytesMut};

use super::{error::Error, from_bytes::FromBytes, to_bytes::ToBytes};

///Fields not supported by some old kafka version
#[derive(Debug)]
pub enum Optional<T> {
    Some(T),
    None,
}

impl<T> Optional<T> {
    pub fn map<F, U>(self, func: F) -> Optional<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Optional::None => Optional::None,
            Optional::Some(val) => Optional::Some(func(val)),
        }
    }
    pub fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::None => false,
        }
    }
    pub fn try_into<R>(self) -> Result<Optional<R>, Error>
    where
        T: Default + TryInto<R, Error = Error>,
        R: Default,
    {
        match self {
            Optional::Some(x) => {
                let v = x.try_into()?;
                Ok(Optional::Some(v))
            }
            Optional::None => Ok(Optional::None),
        }
    }
    pub fn into<R>(self) -> Optional<R>
    where
        T: Default + Into<R>,
        R: Default,
    {
        match self {
            Optional::Some(x) => {
                let v = x.into();
                Optional::Some(v)
            }
            Optional::None => Optional::None,
        }
    }
}
impl<V, E> Optional<Result<V, E>> {
    pub fn wrap_result(self) -> Result<Optional<V>, E> {
        match self {
            Optional::None => Ok(Optional::None),
            Optional::Some(result) => match result {
                Ok(val) => Ok(Optional::Some(val)),
                Err(e) => Err(e),
            },
        }
    }
}

impl<T> Default for Optional<T>
where
    T: Default,
{
    fn default() -> Self {
        Optional::None
    }
}
impl<T> FromBytes for Optional<T>
where
    T: FromBytes,
{
    fn deserialize(buf: &mut Bytes) -> Self {
        Optional::Some(T::deserialize(buf))
    }
}
impl<T> ToBytes for Optional<T>
where
    T: ToBytes + Default,
{
    fn serialize(&self, buf: &mut BytesMut) {
        match self {
            Self::Some(value) => T::serialize(value, buf),
            Self::None => T::serialize(&T::default(), buf),
        }
    }
}
