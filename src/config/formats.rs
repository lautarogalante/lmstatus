use serde::Deserialize;


#[derive(Deserialize)]
pub struct Formats {
    pub date_format: String,
    pub time_format: String,
}