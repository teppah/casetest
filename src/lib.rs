use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use ansi_term::Colour::{Black, Blue, Green, Red, White};
use ansi_term::Style;
use clap::ArgMatches;

pub fn compile(source_file: &str, output_file: &str) -> std::io::Result<Output> {
    Command::new("gcc")
        .arg(source_file)
        .arg("-o")
        .arg(output_file)
        .output()
}

pub fn get_files(args: &ArgMatches) -> (String, String, String) {
    let c_file = args.value_of("file").unwrap();
    let test_file = args.value_of("cases").unwrap();


    (c_file.to_string(), test_file.to_string(), strip(c_file).to_string())
}

pub fn execute_test_cases(compiled_file: &str, mut lines: core::str::Lines) -> TestResult {
    let mut failed: u32 = 0;
    let mut successful: u32 = 0;

    let before = Instant::now();
    for i in 1..=(lines.clone().count() / 2) {
        let mut exec = Command::new(format!("./{}", compiled_file))
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
                true => (String::from_utf8(out.stdout).unwrap().trim().to_string(), true),
                false => {
                    let err = String::from_utf8(out.stderr)
                        .unwrap()
                        .trim()
                        .to_string();
                    (err, false)
                }
            }
        };

        let is_success = (expected_output == output) && is_success;

        let mut msg = String::new();
        match is_success {
            true => {
                msg.push_str(&Green.paint("✔ ").to_string());
                msg.push_str(&format!(" Run #{} {} ", i, Green.paint("success")));
                msg.push_str(
                    &White.dimmed().paint(
                        &format!("(input: {})", test_input))
                        .to_string());
                successful += 1;
            }
            false => {
                msg.push_str(&Red.paint("✗ ").to_string());
                msg.push_str(&format!(" Run #{} {} ",
                                      i,
                                      Red.bold().underline().paint("failed")));
                msg.push_str(&format!("{}\n",
                                      &White.dimmed().paint(
                                          &format!(" (input: {})", test_input))
                                          .to_string()));
                msg.push('\n');
                msg.push_str(&format!("\t{}:    {}\n",
                                      &White.dimmed().paint("Expected"),
                                      expected_output));
                msg.push_str(&format!("\t{}:         {}",
                                      &White.dimmed().paint("Got").to_string(),
                                      output));
                msg.push('\n');
                failed += 1;
            }
        };
        println!("{}", msg);
    }
    let total_time = before.elapsed().as_millis();

    TestResult { passed: successful, failed, total_time_ms: total_time }
}

pub struct TestResult {
    pub passed: u32,
    pub failed: u32,
    pub total_time_ms: u128,
}

fn strip(file: &str) -> &str {
    let last_index = file.rfind(".").unwrap();
    let (stripped_filename, _) = file.split_at(last_index);
    stripped_filename
}
