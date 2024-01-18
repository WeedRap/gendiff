use gendiff::cli::parse_args;

fn main() {
    let (file1, file2, output_format) = parse_args();
    println!("{}, {}, {}", file1, file2, output_format);
}
