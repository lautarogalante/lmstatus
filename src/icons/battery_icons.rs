use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct BatteryIcons {
    pub battery_full: String,
    pub battery_middle: String,
    pub battery_quarter: String,
}