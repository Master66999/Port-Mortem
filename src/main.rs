use clap::Parser;
use roman::{from_roman, to_roman, RomanError};
use std::process::ExitCode;

/// convert between roman and arabic numerals
#[derive(Parser, Debug)]
#[command(name = "roman", about = "convert between roman and arabic numerals")]
struct Args {
    /// the value to convert
    number: String,

    /// convert roman to numeral (case insensitive)
    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if args.reverse {
        match from_roman(&args.number) {
            Ok(r) => {
                println!("{}", r);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
        }
    } else {
        match args.number.parse::<i32>() {
            Ok(i) => match to_roman(i) {
                Ok(n) => {
                    println!("{}", n);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{}", e);
                    ExitCode::FAILURE
                }
            },
            Err(_) => {
                eprintln!("{}", RomanError::NotInteger("decimals cannot be converted".to_string()));
                ExitCode::FAILURE
            }
        }
    }
}
