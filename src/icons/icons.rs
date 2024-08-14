use::std::io;
use serde::Deserialize;

use super::battery_icons::BatteryIcons;
use super::volume_icons::VolumeIcons;
use crate::config::config::Config;

#[derive(Deserialize)]
pub struct Icons {
    pub date: String,
    pub time: String,
    pub battery: BatteryIcons,
    pub volume: VolumeIcons,
}

impl Icons {
    pub fn new(config: &Config) -> Result<Self, io::Error> {
        let battery_icons = BatteryIcons {
            battery_full: config.icons.battery.battery_full.to_string(),
            battery_middle: config.icons.battery.battery_middle.to_string(),
            battery_quarter: config.icons.battery.battery_quarter.to_string(),
        };
        let volume_icons = VolumeIcons {
            volume_high: config.icons.volume.volume_high.to_string(),
            volume_low: config.icons.volume.volume_low.to_string(),
        };
        let icons = Icons {
            date: config.icons.date.to_string(),
            time: config.icons.time.to_string(),
            battery: battery_icons,
            volume: volume_icons,
        };
        Ok(icons)
    }
}