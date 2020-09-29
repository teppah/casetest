use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use ansi_term::Colour::{Green, Red, White};
use clap::ArgMatches;

pub fn compile(source_file: &str, output_file: &str) -> std::io::Result<Output> {
    Command::new("gcc")
        .arg(source_file)
        .arg("-o")
        .arg(output_file)
        .output()
}

pub struct FileNames {
    pub c_file: String,
    pub test_file: String,
    pub compiled_file: String,
}

pub fn get_files(args: &ArgMatches) -> FileNames {
    let c_file = args.value_of("file").unwrap();
    let test_file = args.value_of("cases").unwrap();

    FileNames {
        c_file: c_file.to_string(),
        test_file: test_file.to_string(),
        compiled_file: strip(c_file).to_string(),
    }
}

// TODO: make this return Result
pub fn execute_test_cases(compiled_file: &str, mut lines: core::str::Lines) -> TestResult {
    let mut failed: u32 = 0;
    let mut successful: u32 = 0;

    let before = Instant::now();
    for i in 1..=(lines.clone().count() / 2) {
        let mut exec = Command::new(format!("./{}", compiled_file))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run program");

        let test_input = lines.next().unwrap().trim();
        let expected_output = lines.next().unwrap().trim();

        let (output, is_success) = {
            let stdin = match exec.stdin.as_mut() {
                Some(stdin) => stdin,
                None => {
                    println!("{} Run #{} {}: {}",
                             Red.paint("✗"),
                             i,
                             Red.bold().underline().paint("failed to execute"),
                             White.dimmed().paint("failed to open stdin"));
                    failed += 1;
                    continue;
                }
            };
            match stdin.write_all(test_input.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("{} Run #{} {}: {}",
                             Red.paint("✗"),
                             i,
                             Red.bold().underline().paint("failed to execute"),
                             White.dimmed().paint(format!("{}", e)));
                    failed += 1;
                    continue;
                }
            };
            let out = exec.wait_with_output().unwrap();
            match out.status.success() {
                true => {
                    let output = String::from_utf8(out.stdout).unwrap().trim().to_string();
                    let success = output == expected_output;
                    (output, success)
                }
                false => {
                    let err = String::from_utf8(out.stderr)
                        .unwrap()
                        .trim()
                        .to_string();
                    (err, false)
                }
            }
        };

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
    let last_index = file.rfind('.').unwrap();
    let (stripped_filename, _) = file.split_at(last_index);
    stripped_filename
}
