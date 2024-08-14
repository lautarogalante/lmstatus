use crate::status::status::Status;

use super::icons::Icons;

pub struct SelectedIcons {
    pub date: String,
    pub time: String,
    pub battery: String,
    pub volume: String,
} 

impl SelectedIcons {
    pub fn new(status: &Status, icons: Icons) -> Self {
       return Self::select_icons(&status, icons);
    }
    
    fn select_icons(status: &Status, icons: Icons) -> Self {
        let mut battery_icon = String::new();
        let mut volume_icon = String::new();
        
        if status.battery >= 50 && status.battery <= 100 {
            battery_icon = icons.battery.battery_full;
        } else if status.battery >= 25 && status.battery < 50 {
            battery_icon = icons.battery.battery_middle;
        } else if status.battery < 25 {
            battery_icon = icons.battery.battery_quarter;
        }

        let volume_percent = status.volume;

        if volume_percent > 70 {
            volume_icon = icons.volume.volume_high;
        } else if volume_percent <= 70 {
            volume_icon = icons.volume.volume_low;
        }

        let select_icons = SelectedIcons {
            date: icons.date,
            time: icons.time,
            battery: battery_icon, 
            volume: volume_icon,
        };

        return select_icons;
    }
} 
