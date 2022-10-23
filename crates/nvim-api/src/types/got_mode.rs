use nvim_types::{conversion, serde::Deserializer, Dictionary, Object};
use serde::Deserialize;

use super::Mode;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct GotMode {
    pub blocking: bool,
    pub mode: Mode,
}

impl TryFrom<Dictionary> for GotMode {
    type Error = conversion::Error;
    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(Object::from(dict)))
            .map_err(Into::into)
    }
}
