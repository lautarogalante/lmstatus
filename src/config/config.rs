use crate::prelude::*;
use super::formats::Formats;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub format: Formats,
    pub icons: Icons,
}

pub const CONFIG_DIR: &str = ".config/lmstatus";
pub const CONFIG_FILE: &str = "Config.toml";

impl Config {
    pub fn load(&mut self) -> Result<(), CliError> {
        let config_path = create_path().unwrap();
        let read_path = fs::read_to_string(config_path)?;
        *self = toml::from_str(&read_path).expect("Error in Deserialization.");
        Ok(()) 
    }
}

pub fn create_path() -> Result<PathBuf, CliError> {
    let home_dir = env::var("HOME").expect("environment variable $HOME not found.");
    let config_path = Path::new(&home_dir).join(CONFIG_DIR).join(CONFIG_FILE);
    Ok(config_path)
}
