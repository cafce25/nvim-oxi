use std::path::PathBuf;

use nvim_types::{
    conversion,
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

use crate::Buffer;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AutocmdCallbackArgs {
    /// The `Buffer` specified by `<abuf>`.
    #[serde(rename = "buf")]
    pub buffer: Buffer,

    /// Arbitrary data passed to
    /// [`nvim_oxi::api::exec_autocmds`](crate::exec_autocmds).
    #[serde(default)]
    pub data: Object,

    /// The name of the event that triggered the autocommand.
    pub event: String,

    /// The expanded value of `<afile>`.
    pub file: PathBuf,

    /// The `id` of the autocommand group that the autocommand belongs to, if
    /// any.
    #[serde(default)]
    pub group: Option<u32>,

    /// The `id` of the autocommand.
    pub id: u32,

    /// The expanded value of `<amatch>`.
    pub r#match: String,
}

impl TryFrom<Object> for AutocmdCallbackArgs {
    type Error = conversion::Error;
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl luajit_bindings::Poppable for AutocmdCallbackArgs {
    unsafe fn pop(
        lstate: *mut luajit_bindings::ffi::lua_State,
    ) -> Result<Self, luajit_bindings::Error> {
        let obj = Object::pop(lstate)?;

        Self::try_from(obj)
            .map_err(luajit_bindings::Error::pop_error_from_err::<Self, _>)
    }
}
