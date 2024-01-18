use crate::file_data_loader::DataFormat;
use serde_yaml;

pub fn parse(str_data: String, data_format: DataFormat) -> serde_json::Value {
    match data_format {
        DataFormat::Yaml => serde_yaml::from_str(&*str_data).unwrap(),
        DataFormat::Json => serde_json::from_str(&*str_data).unwrap(),
    }
}
