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
    let mut runs: u8 = 0;
    for _ in 0..(lines.clone().count() / 2) {
        runs += 1;
        let mut exec = Command::new(format!("./{}", stripped_filename))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let test_input = lines.next().unwrap().trim();
        let expected_output = lines.next().unwrap().trim();
        let stdin = exec.stdin.as_mut().expect("Failed to open stdin");

        stdin.write_all(test_input.as_bytes()).expect("failed to write");

        let (output, is_success) = {
            let out = exec.wait_with_output().unwrap();
            match out.status.success() {
                true => (String::from_utf8(out.stdout).unwrap(), true),
                false => {
                    let err = String::from_utf8(out.stderr)
                        .unwrap()
                        .trim()
                        .to_string();
                    (err, false)
                }
            }
        };
        println!("---");
        println!("Run #{}: {}", runs, is_success.to_string().to_uppercase());
        println!("Input: {}", test_input);
        println!("Expected: {:?}", expected_output);
        println!("Actual: {:?}", output.trim());
    }
}

