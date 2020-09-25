use clap::{App, Arg};
use std::fs;
use std::process::Command;

fn main() {
    let app = App::new("casetest")
        .version("1.0")
        .author("Yi Feng Yang <yifeng@yfyang.dev>")
        .about("A tool to run unit tests against single-file C programs")
        .arg(Arg::with_name("file")
            .value_name("PROGRAM")
            .help("The file to run test cases against")
            .index(1)
            .required(true))
        .arg(Arg::with_name("cases")
            .value_name("TESTS")
            .help("A plaintext file with test cases on odd lines and expected output on even lines")
            .index(2)
            .required(true));
    let matches = app.get_matches();

    let c_file = matches.value_of("file").unwrap();
    let test_file = matches.value_of("cases").unwrap();

    let test_cases = match fs::read_to_string(test_file) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Failed to read test file '{}': {}", test_file, e);
            return;
        }
    };

    let last_index = c_file.rfind(".").unwrap();
    let (stripped_filename, _) = c_file.split_at(last_index);

    let gcc = Command::new("gcc")
        .arg(c_file)
        .arg("-o")
        .arg(stripped_filename)
        .output()
        .expect("Failed to execute gcc");
    println!("{:?}", gcc)
}
