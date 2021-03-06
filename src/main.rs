use std::fs;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use clap::{App, Arg, SubCommand, AppSettings};

use casetest::{compile, execute_test_cases, TestFileNames, get_test_files, TestResult};

fn main() {
    #[cfg(target_os = "windows")]
        let enabled = ansi_term::enable_ansi_support();

    let app = App::new("casetest")
        .version("1.0")
        .author("teppah <teppah@yfyang.dev>")
        .about("A tool to run unit tests against single-file C programs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("test")
                .about("Run unit tests on a C file with a given input file")
                .arg(Arg::with_name("file")
                    .value_name("PROGRAM")
                    .help("The file to run test cases against")
                    .index(1)
                    .required(true))
                .arg(Arg::with_name("cases")
                    .value_name("TESTS")
                    .help("A plaintext file with test cases on odd lines and expected output on even lines")
                    .index(2)
                    .required(true)))
        .subcommand(
            SubCommand::with_name("watch")
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("test", Some(test_matches)) => {
            let TestFileNames { c_file, test_file, compiled_file } = get_test_files(&test_matches);
            println!("{} Testing file \"{}\" against inputs in \"{}\"",
                     Blue.bold().paint("🛈"),
                     Yellow.bold().paint(c_file),
                     Yellow.bold().paint(test_file));
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
            };

            let test_cases = match fs::read_to_string(&test_file) {
                Ok(str) => str,
                Err(e) => {
                    eprintln!("Failed to read test file '{}': {}", test_file, e);
                    return;
                }
            };

            let lines = test_cases.lines();
            let TestResult { passed, failed, total_time_ms } = execute_test_cases(&compiled_file, lines);

            println!("{}", Blue.paint("------Summary------"));
            let total = failed + passed;
            println!("Tests: \n\t○ {} total\n\t{}\n\t{}",
                     total,
                     Red.blink().paint(format!("✗ {} failed", failed)),
                     Green.paint(format!("✔ {} passed", passed)));
            println!("Time elapsed: {} ms", total_time_ms);
        }
        ("watch", Some(matches)) => {}
        _ => panic!("Should be unreachable")
    }
}

