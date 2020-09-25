use clap::{App, Arg};

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

    println!("{} {}", c_file, test_file);

    
}
