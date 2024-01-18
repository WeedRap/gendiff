use crate::diff_tree::{DiffNode, DiffType};

fn to_str(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Object(_obj) => "[complex value]".to_string(),
        serde_json::Value::String(s) => format!("'{}'", s.clone()),
        _ => value.to_string(),
    }
}

fn get_property_name(key: &str, path: &str) -> String {
    if path != "" {
        format!("{}.{}", path, key)
    } else {
        key.to_string()
    }
}

pub fn render_plain<'a>(diff: &Vec<DiffNode>) -> String {
    fn _iter(diff: &Vec<DiffNode>, path: String) -> Vec<String> {
        let mut result = Vec::new();

        for elem in diff {
            match &elem.node_type {
                DiffType::Deleted { key, value: _ } => {
                    result.push(format!(
                        "Property '{}' was removed",
                        get_property_name(key, &path)
                    ));
                }
                DiffType::Added { key, value } => {
                    result.push(format!(
                        "Property '{}' was added with value: {}",
                        get_property_name(key, &path),
                        to_str(value)
                    ));
                }
                DiffType::Changed {
                    key,
                    value1,
                    value2,
                } => {
                    result.push(format!(
                        "Property '{}' was updated. From {} to {}",
                        get_property_name(key, &path),
                        to_str(value1),
                        to_str(value2)
                    ));
                }
                DiffType::Nested { key, children } => {
                    result.extend(_iter(children, get_property_name(key, &path)));
                }
                _ => {}
            }
        }
        result
    }
    _iter(diff, "".to_string()).join("\n")
}
