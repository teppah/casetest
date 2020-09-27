use std::error::Error;
use std::fs;
use std::io::{BufWriter, Read, Stdin, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use ansi_term::Colour::{Black, Blue, Green, Red, White};
use clap::{App, Arg};

use casetest::{compile, execute_test_cases, get_files, TestResult};

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
    let (c_file, test_file, compiled_file) = get_files(&matches);

    match compile(&c_file, &compiled_file) {
        Ok(out) => {
            if !out.status.success() {
                eprintln!("Failed to compile with gcc:\n{}", String::from_utf8_lossy(&out.stderr));
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to execute compiler:\n{}", e);
            return;
        }
    }


    let test_cases = match fs::read_to_string(&test_file) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("Failed to read test file '{}': {}", test_file, e);
            return;
        }
    };


    let mut lines = test_cases.lines();
    let TestResult { passed, failed, total_time_ms } = execute_test_cases(&compiled_file, lines);

    println!("{}", Blue.paint("------Summary------"));
    let total = failed + passed;
    println!("Tests: \n\t○ {} total\n\t{}\n\t{}",
             total,
             Red.blink().paint(format!("✗ {} failed", failed)),
             Green.paint(format!("✔ {} passed", passed)));
    println!("Time elapsed: {} ms", total_time_ms);
}

