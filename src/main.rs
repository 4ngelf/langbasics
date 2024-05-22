use rot13::Rot13;
use std::{env, io};

enum Operation {
    Normal,
    OnlyStdin,
    Help,
    Version,
}

fn parse_args(args: &Vec<String>) -> Operation {
    if args.is_empty() {
        return Operation::OnlyStdin;
    }

    for option in args {
        match option.as_str() {
            "-h" | "--help" => return Operation::Help,
            "-v" | "--version" => return Operation::Version,
            "--" => return Operation::Normal,
            _ => continue,
        }
    }

    Operation::Normal
}

fn encode_stdin() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => println!("{}", line.rot13()),
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn encode_args(args: &Vec<String>) {
    for arg in args {
        match arg.as_str() {
            "-" => encode_stdin(),
            "--" => continue,
            _ => println!("{}", arg.rot13()),
        }
    }
}

fn show_help() {
    println!(
        "ROT13 enconding utility

Usage: rot13 [TEXT]...

Options:
  -v, --version             Print version info and exit
  -h, --help                Print help"
    );
}

fn show_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_args(&args) {
        Operation::Normal => encode_args(&args),
        Operation::OnlyStdin => encode_stdin(),
        Operation::Help => show_help(),
        Operation::Version => show_version(),
    }
}
