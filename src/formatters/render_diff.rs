use crate::{
    diff_tree::DiffNode, formatters::plain::render_plain, formatters::stylish::render_stylish,
};

pub fn render_diff(diff: &Vec<DiffNode>, format_name: String) -> String {
    match format_name.as_str() {
        "stylish" => render_stylish(diff),
        "plain" => render_plain(diff),
        _ => panic!(""),
    }
}
