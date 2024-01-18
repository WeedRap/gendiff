use crate::diff_tree::DiffNode;
use crate::formatters::plain::render_plain;
use crate::formatters::stylish::to_stylish;

pub fn render_diff(diff: &Vec<DiffNode>, format_name: String) -> String {
    match format_name.as_str() {
        "stylish" => to_stylish(diff),
        "plain" => render_plain(diff),
        _ => panic!(""),
    }
}
