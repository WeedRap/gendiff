use crate::{
    diff_tree::build_diff, file_data_loader::get_file_data, formatters::render_diff::render_diff,
};

pub fn generate_diff(file1: String, file2: String, output_format: String) -> String {
    let data1 = get_file_data(file1);
    let data2 = get_file_data(file2);

    let diff = build_diff(&data1, &data2);
    render_diff(&diff, output_format)
}
