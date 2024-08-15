use crate::prelude::*;

const CONFIG_DIR: &str = ".config/lmstatus";
const CONFIG_FILE: &str = "Config.toml";

pub fn load_config() -> Result<Config, CliError> {
    let home_dir = env::var("HOME").expect("environment variable $HOME not found.");
    let config_path = Path::new(&home_dir).join(CONFIG_DIR).join(CONFIG_FILE);
    let read_path = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&read_path).expect("Error in Deserialization.");
    Ok(config)
}

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