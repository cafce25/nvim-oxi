use nvim_types::{
    conversion,
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

/// Attributes related to a highlight group.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct HighlightInfos {
    pub background: Option<u32>,
    pub bg_indexed: Option<bool>,
    pub blend: Option<u32>,
    pub bold: Option<bool>,
    pub fg_indexed: Option<bool>,
    pub foreground: Option<u32>,
    pub italic: Option<bool>,
    pub reverse: Option<bool>,
    pub special: Option<u32>,
    pub standout: Option<bool>,
    pub strikethrough: Option<bool>,
    pub undercurl: Option<bool>,
    pub underdash: Option<bool>,
    pub underdot: Option<bool>,
    pub underline: Option<bool>,
    pub underlineline: Option<bool>,
}

impl TryFrom<Object> for HighlightInfos {
    type Error = conversion::Error;
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
