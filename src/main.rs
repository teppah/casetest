use clap::{App, Arg};
use std::fs;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{Stdin, BufWriter, Write, Read};

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


    let last_index = c_file.rfind(".").unwrap();
    let (stripped_filename, _) = c_file.split_at(last_index);

    let gcc = match Command::new("gcc")
        .arg(c_file)
        .arg("-o")
        .arg(stripped_filename)
        .output() {
        Ok(gcc) => gcc,
        Err(e) => {
            eprintln!("Failed to execute gcc:\n{}", e);
            return;
        }
    };
    if !gcc.status.success() {
        eprintln!("Failed to compile with gcc:\n{}", String::from_utf8_lossy(&gcc.stderr));
        return;
    }

    let test_cases = match fs::read_to_string(test_file) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Failed to read test file '{}': {}", test_file, e);
            return;
        }
    };
    let mut lines = test_cases.lines();
    for _ in 0..lines.clone().count() {
        println!("iteration");
        let run = Command::new(format!("./{}", stripped_filename))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        println!("iteration2");
        let mut stdout = run.stdout.unwrap();
        let mut stdin = run.stdin.unwrap();
        println!("iteration3");

        let input = lines.next().unwrap();
        let expected_output = lines.next().unwrap();
        println!("iteration4");

        stdin.write_all(input.as_bytes());
        stdin.flush();

        println!("iteration5");
        let mut actual_output = String::new();

        println!("iteration6");
        stdout.read_to_end()
        stdout.read_to_string(&mut actual_output);

        println!("----output for \"{}\" as input----", input);
        println!("---expected output: {}", expected_output);
        println!("{}", actual_output);
    }
}

