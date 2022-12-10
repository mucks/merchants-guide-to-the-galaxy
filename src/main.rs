mod error;
mod merchants_guide;
mod result;
mod roman;

use result::Result;
use std::{
    env, fs,
    io::{BufRead, BufReader},
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

fn run_interactive() {}

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
