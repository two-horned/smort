use std::io::{stdin, Result, stdout, Write};
//use std::f64::consts::{PI, E};
use smort::{help, is_quit, is_help};
use smort::syntax::is_legal;
use smort::calculator::{calculate, is_two_signs};
fn main() -> Result<()> {
        println!("Enter 'h' for help");
    loop {
        print!(": ");
        stdout().flush()?;

        let mut e = String::new();
        stdin().read_line(&mut e)?;
        e.retain(|c| c != ' ');

        let e = {
            while is_two_signs(&e) {
                e = e.replace("+-", "-");
                e = e.replace("-+", "-");
                e = e.replace("--", "+");
                e = e.replace("++", "+");
            }
            e.trim()
        };
        //let e = e.replace("pi", &PI.to_string());
        //let e = e.replace("e", &E.to_string());

        if is_quit(e) {
            break;
        } else if is_help(e) {
            help();
        } else if is_legal(e) {
            println!("{} = {}", e, calculate(e));
        } else {
            println!("Wrong input. Deleting your filesystem...")
        }

    }
    println!("Smort Calculator by Said Kadrioski");
    Ok(())
}
