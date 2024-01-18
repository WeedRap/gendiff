use gendiff::{cli::parse_cli_args, differ::generate_diff};

fn main() {
    let (file1, file2, output_format) = parse_cli_args();
    println!("{}", generate_diff(file1, file2, output_format));
}
