use serde::Deserialize;

use super::formats::Formats;
use crate::Icons;

#[derive(Deserialize)]
pub struct Config {
    pub format: Formats,
    pub icons: Icons,
}


