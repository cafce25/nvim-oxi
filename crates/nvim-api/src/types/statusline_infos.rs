use nvim_types::{
    conversion,
    serde::Deserializer,
    Object, Dictionary,
};
use serde::Deserialize;

use super::StatuslineHighlightInfos;

/// Statusline informations returned by
/// [`eval_statusline`](crate::eval_statusline).
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineInfos {
    /// Vector of highlight informations for the statusline populated if the
    /// [`highlights`](crate::opts::EvalStatuslineOptsBuilder::highlights)
    /// field of  was set to `true`.
    #[serde(default)]
    pub highlights: Vec<StatuslineHighlightInfos>,

    /// Characters displayed in the statusline.
    pub str: String,

    /// Display width of the statusline.
    pub width: u32,
}

impl TryFrom<Dictionary> for StatuslineInfos {
    type Error = conversion::Error;
    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
        Ok(Self::deserialize(Deserializer::new(Object::from(dict)))?)
    }
}
