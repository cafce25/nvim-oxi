//! Traits for converting between Neovim [`Object`]s and Rust types.

use std::collections::HashMap;

use thiserror::Error as ThisError;

use crate::{
    Array, Boolean, Dictionary, Float, Function, Integer, Object, ObjectKind,
};

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
pub enum Error {
    #[error("Was expecting a \"{expected}\" but received a \"{actual}\"")]
    FromWrongType { expected: &'static str, actual: &'static str },

    #[error(transparent)]
    FromInt(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "serde")]
    #[error(transparent)]
    Serde(#[from] crate::serde::Error),
}

impl TryFrom<Object> for () {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Nil => Ok(()),

            other => Err(Error::FromWrongType {
                expected: "nil",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Boolean {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Boolean => Ok(unsafe { obj.as_boolean_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "bool",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Integer
            | ObjectKind::Buffer
            | ObjectKind::Window
            | ObjectKind::TabPage => Ok(unsafe { obj.as_integer_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "integer",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Float {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Float => Ok(unsafe { obj.as_float_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "float",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for crate::String {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::String => Ok(unsafe { obj.into_string_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Array {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Array => Ok(unsafe { obj.into_array_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl TryFrom<Object> for Dictionary {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::Dictionary => Ok(unsafe { obj.into_dict_unchecked() }),

            other => Err(Error::FromWrongType {
                expected: "string",
                actual: other.as_static(),
            }),
        }
    }
}

impl<A, R> TryFrom<Object> for Function<A, R> {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        match obj.kind() {
            ObjectKind::LuaRef => {
                Ok(Self::from_ref(unsafe { obj.as_luaref_unchecked() }))
            },

            other => Err(Error::FromWrongType {
                expected: "function",
                actual: other.as_static(),
            }),
        }
    }
}

/// Implements `TryFrom<Object>` for a type that implements `From<Integer>`.
macro_rules! from_int {
    ($integer:ty) => {
        impl TryFrom<Object> for $integer {
            type Error = Error;
            fn try_from(obj: Object) -> Result<Self, Error> {
                Integer::try_from(obj).map(Into::into)
            }
        }
    };
}

from_int!(i128);

/// Implements `TryFrom<Object>` for a type that implements `TryFrom<Integer>`.
macro_rules! try_from_int {
    ($integer:ty) => {
        impl TryFrom<Object> for $integer {
            type Error = Error;
            fn try_from(obj: Object) -> Result<Self, Error> {
                Integer::try_from(obj).and_then(|n| Ok(n.try_into()?))
            }
        }
    };
}

try_from_int!(i8);
try_from_int!(u8);
try_from_int!(i16);
try_from_int!(u16);
try_from_int!(i32);
try_from_int!(u32);
try_from_int!(u64);
try_from_int!(u128);
try_from_int!(isize);
try_from_int!(usize);

impl TryFrom<Object> for f32 {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        Ok(Float::try_from(obj)? as _)
    }
}

impl TryFrom<Object> for String {
    type Error = Error;
    fn try_from(obj: Object) -> Result<Self, Error> {
        crate::String::try_from(obj)
            .and_then(|nvim_str| Ok(nvim_str.into_string()?))
    }
}

/// Implements `ToObject` for "big integer" types.
macro_rules! bigint_to_obj {
    ($type:ty) => {
        impl TryFrom<$type> for Object {
            type Error = Error;
            fn try_from(i: $type) -> Result<Object, Error> {
                Ok(i64::try_from(i)?.into())
            }
        }
    };
}

bigint_to_obj!(u64);
bigint_to_obj!(isize);
bigint_to_obj!(usize);
bigint_to_obj!(i128);
bigint_to_obj!(u128);

impl<T> TryFrom<Vec<T>> for Object
where
    T: TryInto<Object>,
    Error: From<T::Error>,
{
    type Error = Error;
    fn try_from(items: Vec<T>) -> Result<Object, Error> {
        Ok(items
            .into_iter()
            .map(TryInto::try_into)
            .map(|x| x.map_err(Into::into))
            .collect::<Result<Array, Error>>()?
            .into())
    }
}

impl<K, V> TryFrom<HashMap<K, V>> for Object
// impl<K, V> ToObject for HashMap<K, V>
where
    K: Into<crate::String>,
    V: TryInto<Object>,
    Error: From<V::Error>,
{
    type Error = Error;
    fn try_from(map: HashMap<K, V>) -> Result<Object, Error> {
        map.into_iter()
            .map(|(k, v)| Ok((k, v.try_into()?)))
            .collect::<Result<Dictionary, Error>>()
            .map(Into::into)
    }
}
