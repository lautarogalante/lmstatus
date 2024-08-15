use crate::prelude::*;
use super::formats::Formats;

#[derive(Deserialize)]
pub struct Config {
    pub format: Formats,
    pub icons: Icons,
}


