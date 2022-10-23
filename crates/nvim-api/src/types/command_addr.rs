use nvim_types::{
    conversion,
    serde::Serializer,
    Object,
};
use serde::{Deserialize, Serialize};

/// See `:h command-addr` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandAddr {
    Lines,
    Arguments,
    Buffers,
    LoadedBuffers,
    Windows,
    Tabs,
    Quickfix,
    Other,
}

impl TryFrom<CommandAddr> for Object {
    type Error = conversion::Error;
    fn try_from(addr: CommandAddr) -> Result<Object, Self::Error> {
        addr.serialize(Serializer::new()).map_err(Into::into)
    }
}
