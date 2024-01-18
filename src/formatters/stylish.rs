use crate::diff_tree::{DiffNode, DiffType};

fn build_indent(indent: usize) -> String {
    " ".repeat(indent)
}

fn to_str(value: &serde_json::Value, indent: usize) -> String {
    match value {
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Object(obj) => {
            let lines: Vec<String> = obj
                .iter()
                .map(|(key, val)| {
                    format!(
                        "{}  {}: {}",
                        build_indent(indent),
                        key,
                        to_str(val, indent + 4)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", lines.join("\n"), build_indent(indent - 2))
        }
        serde_json::Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}

pub fn render_stylish<'a>(diff: &Vec<DiffNode>) -> String {
    fn _iter(data: &Vec<DiffNode>, indent: usize) -> String {
        let mut result = Vec::new();

        for elem in data {
            match &elem.node_type {
                DiffType::Deleted { key, value } => {
                    result.push(format!(
                        "{}- {}: {}",
                        build_indent(indent),
                        key,
                        to_str(value, indent + 4)
                    ));
                }
                DiffType::Added { key, value } => {
                    result.push(format!(
                        "{}+ {}: {}",
                        build_indent(indent),
                        key,
                        to_str(value, indent + 4)
                    ));
                }
                DiffType::Changed {
                    key,
                    value1,
                    value2,
                } => {
                    result.push(format!(
                        "{}- {}: {}",
                        build_indent(indent),
                        key,
                        to_str(value1, indent + 4)
                    ));
                    result.push(format!(
                        "{}+ {}: {}",
                        build_indent(indent),
                        key,
                        to_str(value2, indent + 4)
                    ));
                }
                DiffType::Unchanged { key, value } => {
                    result.push(format!(
                        "{}  {}: {}",
                        build_indent(indent),
                        key,
                        to_str(value, indent + 4)
                    ));
                }
                DiffType::Nested { key, children } => {
                    result.push(format!(
                        "{}  {}: {}",
                        build_indent(indent),
                        key,
                        _iter(children, indent + 4)
                    ));
                }
            }
        }

        let result_string = result.join("\n");
        format!("{{\n{}\n{}}}", result_string, build_indent(indent - 2))
    }
    _iter(diff, 2)
}
