use crate::prelude::*;
use crate::prelude::utils::*;

pub struct Status {
    pub(crate) date: String,
    pub(crate) time: String,
    pub(crate) battery: i16,
    pub(crate) volume: i16,
}

impl Status {
    pub fn new(config: &Config) -> Result<String, CliError> {
        let status = Self::create_status(&config)?;
        let icons = SelectedIcons::new(&status,Icons::new(config).map_err(CliError::Io)?);
        let format = format!(
            "{}{}% | {}{}% | {}{} | {}{} |",
            icons.battery,
            status.battery,
            icons.volume,
            status.volume,
            icons.time,
            status.time,
            icons.date,
            status.date,
        );
        Ok(format)
    }

    pub fn create_status(config: &Config) -> Result<Self, CliError> {
        let date_format = &config.format.date_format;
        let time_format = &config.format.time_format;
        let date = get_date(date_format.to_string());
        let time = get_time(time_format.to_string());
        let battery = get_battery_status()?;
        let volume = get_current_volume()?;
        Ok(Status {
            date,
            time,
            battery,
            volume,
        })
    }
}