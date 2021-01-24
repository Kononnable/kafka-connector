use std::convert::TryInto;

use bytes::BytesMut;

use super::{error::Error, from_bytes::FromBytes, to_bytes::ToBytes};

///Fields not supported by some old kafka version
pub enum Optional<T>
// TODO: remove requirement of default
// where
//     T: Default,
{
    Some(T),
    None,
}

impl<T> Optional<T>
// where
//     T: Default,
{
    pub fn map<F, U>(self, func: F) -> Optional<U>
    where
        // U: Default,
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
    T: FromBytes + Default,
{
    fn deserialize<R>(buf: &mut R) -> Self
    where
        R: Iterator<Item = u8>,
    {
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
