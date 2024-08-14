use serde::Deserialize;

#[derive(Deserialize)]
pub struct VolumeIcons {
    pub volume_high: String,
    pub volume_low: String,
}
