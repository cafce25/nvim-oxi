use nvim_types::{
    conversion,
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

use super::{ExtmarkHlMode, ExtmarkVirtTextPosition};

/// Extmark infos returned by `Buffer::get_extmark_by_id`.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct ExtmarkInfos {
    #[serde(default)]
    pub end_col: Option<usize>,

    #[serde(default)]
    pub end_right_gravity: Option<bool>,

    #[serde(default)]
    pub end_row: Option<usize>,

    #[serde(default)]
    pub hl_eol: Option<bool>,

    #[serde(default)]
    pub hl_group: Option<String>,

    #[serde(default)]
    pub hl_mode: Option<ExtmarkHlMode>,

    #[serde(default)]
    pub priority: Option<u32>,

    pub right_gravity: bool,

    #[serde(default)]
    pub ui_watched: Option<bool>,

    #[serde(default)]
    pub virt_lines: Option<Vec<Vec<(String, String)>>>,

    #[serde(default)]
    pub virt_lines_above: Option<bool>,

    #[serde(default)]
    pub virt_lines_leftcol: Option<bool>,

    #[serde(default)]
    pub virt_text: Option<Vec<(String, String)>>,

    #[serde(default)]
    pub virt_text_hide: Option<bool>,

    #[serde(default)]
    pub virt_text_pos: Option<ExtmarkVirtTextPosition>,

    #[serde(default)]
    pub virt_text_win_col: Option<i64>,
}

impl TryFrom<Object> for ExtmarkInfos {
    type Error = conversion::Error;
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
