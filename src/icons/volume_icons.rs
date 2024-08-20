use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct VolumeIcons {
    pub volume_high: String,
    pub volume_low: String,
}
