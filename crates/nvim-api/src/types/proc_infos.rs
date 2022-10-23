use nvim_types::{
    conversion,
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ProcInfos {
    pub name: Option<String>,
    pub pid: Option<u32>,
    pub ppid: Option<u32>,
}

impl TryFrom<Object> for ProcInfos {
    type Error = conversion::Error;
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
