mod error;
mod merchants_guide;
mod result;
mod roman;

use crate::result::Result;
use merchants_guide::MerchantsGuide;

fn print_answer(answer: Option<String>) {
    if let Some(msg) = answer {
        println!("{}", msg);
    }
}

fn run() -> Result<()> {
    let mut g = MerchantsGuide::new();

    g.handle_input("glob is I")?;
    g.handle_input("prok is V")?;
    g.handle_input("pish is X")?;
    g.handle_input("tegj is L")?;
    g.handle_input("tonar is C")?;
    g.handle_input("glob glob Silver is 34 Credits")?;
    g.handle_input("glob prok Gold is 57800 Credits")?;
    g.handle_input("pish pish Iron is 3910 Credits")?;
    print_answer(g.handle_input("how much is pish tegj glob glob ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Silver ?")?);
    //print_answer(g.handle_input("how many Credits is glob prok tonar Silver ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Gold ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Iron ?")?);
    print_answer(
        g.handle_input("how much wood could a woodchuck chuck if a woodchuck could chuck wood ?")?,
    );

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        err.print_generic();
    }
}
