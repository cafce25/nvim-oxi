use nvim_types::{
    conversion,
    serde::Serializer,
    Object,
};
use serde::{Deserialize, Serialize};

/// Number of arguments accepted by a command.
#[non_exhaustive]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub enum CommandNArgs {
    #[default]
    #[serde(rename = "0")]
    Zero,

    #[serde(rename = "1")]
    One,

    #[serde(rename = "?")]
    ZeroOrOne,

    #[serde(rename = "+")]
    OneOrMore,

    #[serde(rename = "*")]
    Any,
}

impl TryFrom<CommandNArgs> for Object {
    type Error = conversion::Error;
    fn try_from(args: CommandNArgs) -> Result<Object, Self::Error> {
        args.serialize(Serializer::new()).map_err(Into::into)
    }
}
