mod error;
mod merchants_guide;
mod result;
mod roman;

use result::Result;
use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

use crate::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        run_from_file(path).unwrap();
    } else {
        run_interactive();
    }
}

fn print_instruction() {
    println!("Enter your statement or question below:");
    println!("---");
    print!("-> ");
}

fn run_interactive() {
    println!("Welcome to the Merchants Guide to the Galaxy");
    println!("Enter 'restart' to restart the program and 'exit' to exit");

    print_instruction();

    let mut g = merchants_guide::MerchantsGuide::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().flatten() {
        if line == "restart" {
            g = merchants_guide::MerchantsGuide::new();
            println!("merchants guide restarted!");
            print_instruction();
            continue;
        }
        if line == "exit" {
            println!("merchants guide exited!");
            break;
        }

        match g.handle_input(&line) {
            Ok(out) => match out {
                Some(s) => println!("{}", s),
                None => println!("input accepted!"),
            },
            Err(err) => match err {
                Error::InvalidInput(input) => {
                    println!("The input sentence is invalid: {}", input);
                }
                Error::InvalidRomanValues(input) => {
                    println!("The Roman numerals are invalid: {} ", input);
                }
                _ => {
                    println!("{}", err.into_generic());
                }
            },
        }
        println!("---");
        print_instruction();
    }
}

fn run_from_file(path: &str) -> Result<()> {
    let f = fs::File::open(path).unwrap();

    let mut g = merchants_guide::MerchantsGuide::new();

    for line in BufReader::new(f).lines().flatten() {
        match g.handle_input(&line) {
            Ok(os) => {
                if let Some(s) = os {
                    println!("{}", s);
                }
            }
            Err(err) => {
                let s = match err {
                    Error::Custom(_) => err.into_generic(),
                    _ => "".into(),
                };
                println!("{}", s);
            }
        }
    }

    Ok(())
}
