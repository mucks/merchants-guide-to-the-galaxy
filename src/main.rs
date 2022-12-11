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

// helper function for run_interactive
fn print_instruction() {
    println!("Enter your statement or question below:");
    println!("---");
    print!("-> ");
}

fn run_interactive() {
    println!("Welcome to the Merchants Guide to the Galaxy");
    println!("Enter 'restart' to restart the program and 'exit' to exit");

    print_instruction();

    let mut guide = merchants_guide::MerchantsGuide::new();

    let stdin = io::stdin();

    // waits for input line from stdin and runs the code inside the loop
    for line in stdin.lock().lines().flatten() {
        // if the user inputs 'restart' the merchanst guide values will be reinitialized
        if line == "restart" {
            guide = merchants_guide::MerchantsGuide::new();
            println!("merchants guide restarted!");
            print_instruction();
            continue;
        }
        // if the user inputs 'exit' the merchants program exits
        if line == "exit" {
            println!("merchants guide exited!");
            break;
        }

        // all other input gets fed into the merchants_guide
        match guide.handle_input(&line) {
            // if the input is successful print an answer
            Ok(out) => match out {
                // if the input is a question print the answer
                Some(s) => println!("{}", s),
                // if the input is a statement print a confirmation message!
                None => println!("input accepted!"),
            },
            // if the input in't succesfull print the correct error message
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

// converts a file to roman value
fn run_from_file(path: &str) -> Result<()> {
    // open the file at the path that's specified in the path parameter
    let f = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("path: '{}' not found!", path);
            return Ok(());
        }
    };

    let mut guide = merchants_guide::MerchantsGuide::new();

    // read the file line by line
    for line in BufReader::new(f).lines().flatten() {
        // feed the line into the merchants_guide
        match guide.handle_input(&line) {
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
