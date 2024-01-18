use crate::parser::parse;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum DataFormat {
    Json,
    Yaml,
}

fn get_extension_from_filename(filename: &String) -> DataFormat {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    match extension {
        "json" => DataFormat::Json,
        "yaml" | "yml" => DataFormat::Yaml,
        _ => panic!("Unsupported file extension {}", extension),
    }
}

pub fn get_file_data(file_path: String) -> serde_json::Value {
    let file_content =
        fs::read_to_string(&file_path).expect("LogRocket: Should have been able to read the file");
    let data_format = get_extension_from_filename(&file_path);
    println!("{:#?}", data_format);
    parse(file_content, data_format)
}
