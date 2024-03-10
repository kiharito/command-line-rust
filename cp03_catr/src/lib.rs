use clap::{Arg, ArgAction, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename);
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("kiharito <kiharito@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();
    Ok(Config {
        files,
        number_lines: *matches.get_one("number").unwrap(),
        number_nonblank_lines: *matches.get_one("number_nonblank").unwrap(),
    })
}
