use crate::prelude::*;

pub fn get_date(format: String) -> String {
    let date_format = format;
    Local::now().format(&date_format).to_string()
}

pub fn get_time(format: String) -> String {
    let time_format = format;
    Local::now().format(&time_format).to_string()
}

pub fn get_battery_status() -> Result<i16, CliError> {
    let value = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").map_err(CliError::Io);
    let converted_type: i16 = value?.trim().parse().map_err(CliError::Parse)?;
    Ok(converted_type)
}

pub fn get_current_volume() -> Result<i16, CliError> {
    let percent = Command::new("sh")
        .arg("-c")
        .arg(
            "pactl list sinks | grep -A 15 'Sink #0' | grep 'Volume:' | head -n 1 | awk '{print $5}' | tr -d '%'"
        )
        .output()?;
    let value_percent = percent.stdout;
    let percent = String::from_utf8(value_percent);
    let volume_percent: i16 = percent.unwrap().trim().parse().map_err(CliError::Parse)?;
    Ok(volume_percent)
}