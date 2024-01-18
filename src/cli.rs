use clap::{command, Arg};

pub fn parse_cli_args() -> (String, String, String) {
    let matches = command!()
        .about("Compares two configuration files and shows a difference.")
        .arg(Arg::new("first_file").required(true))
        .arg(Arg::new("second_file").required(true))
        .arg(
            Arg::new("format")
                .help("Set the format of output")
                .short('f')
                .long("format")
                .default_value("stylish"),
        )
        .get_matches();

    let first_file = matches.get_one::<String>("first_file").unwrap();
    let second_file = matches.get_one::<String>("second_file").unwrap();
    let output_format = matches.get_one::<String>("format").unwrap();
    (first_file.to_string(), second_file.to_string(), output_format.to_string())
}