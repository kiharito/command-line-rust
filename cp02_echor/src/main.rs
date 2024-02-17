use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("kiharito <kiharito@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .required(true)
                .action(ArgAction::Append)
                .value_name("TEXT")
                .help("Input text"),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();
    let text: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();
    let omit_newline: bool = *matches.get_one("omit_newline").unwrap();
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
