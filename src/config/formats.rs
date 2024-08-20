use serde::Deserialize;


#[derive(Deserialize, Default, Debug)]
pub struct Formats {
    pub date_format: String,
    pub time_format: String,
}