use gendiff::differ::generate_diff;
use std::fs;

fn get_fixtures_data(file_name: &str) -> String {
    let file_content = fs::read_to_string(get_fixtures_file_path(file_name))
        .expect("Could not read the JSON file for the test");
    file_content
}

fn get_fixtures_file_path(file_name: &str) -> String {
    format!("tests/fixtures/{}", file_name)
}

#[test]
fn test_generate_diff() {
    let test_cases = [
        ("file1.json", "file2.json"),  // json to json
        ("file1.yml", "file2.yml"),  // yml to yml
        ("file1.json", "file2.yml"),  // json to yml
    ];

    for (file_name1, file_name2) in test_cases {
        let (file1, file2) = (
            get_fixtures_file_path(file_name1),
            get_fixtures_file_path(file_name2),
        );
        let stylish_result = get_fixtures_data("result_stylish");
        assert_eq!(
            generate_diff(file1.clone(), file2.clone(), "stylish".to_string()),
            stylish_result
        );
        let plain_result = get_fixtures_data("result_plain");
        assert_eq!(
            generate_diff(file1.clone(), file2.clone(), "plain".to_string()),
            plain_result
        );
    }
}
