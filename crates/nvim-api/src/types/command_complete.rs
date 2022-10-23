use nvim_types::{
    conversion,
    serde::Serializer,
    Function,
    Object,
};
use serde::Serialize;

/// See `:h command-complete` for details.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandComplete {
    Arglist,
    Augroup,
    Buffer,
    Behave,
    Color,
    Command,
    Compiler,
    Cscope,
    Dir,
    Environment,
    Event,
    Expression,
    File,
    FileInPath,
    Filetype,
    Function,
    Help,
    Highlight,
    History,
    Locale,
    Lua,
    Mapclear,
    Mapping,
    Menu,
    Messages,
    Option,
    Packadd,
    Shellcmd,
    Sign,
    Syntax,
    Syntime,
    Tag,
    TagListfiles,
    User,
    Var,

    /// See `:h command-completion-customlist` for details.
    CustomList(Function<(String, String, usize), Vec<String>>),
}

impl TryFrom<CommandComplete> for Object {
    type Error = conversion::Error;
    fn try_from(complete: CommandComplete) -> Result<Object, Self::Error> {
        complete.serialize(Serializer::new()).map_err(Into::into)
    }
}
