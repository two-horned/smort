use smort::fraction::Fraction;

//use std::f64::consts::{PI, E};
use {
    smort::calculator2::{calculate,fraction_to_float},
    std::io::{self, Write},
};

fn credits() {
    println!("Smort Calculator by Said Kadrioski");
}

fn help() {
    println!("Available Commands:");
    println!("    'h': show help,");
    println!("    'd': show last result as decimal,");
    println!("    '[a]': apply following operation on the last result,");
    println!("    'c': show credits,");
    println!("    'q': quit application,");
    println!();
    println!("Available Operants:");
    println!("    + (addition),");
    println!("    - (subtraction),");
    println!("    * (multiplication),");
    println!("    / (division),");
    println!("    ^ (power of),");
    println!("    % (modulo),");
    println!("You can use parenthesis too.");
}

fn main() -> io::Result<()> {
    let mut l = Fraction::new(0,1).unwrap();
    println!("Enter 'h' for help");
    loop {
        print!(": ");
        io::stdout().flush()?;
        let mut e = String::new();
        io::stdin().read_line(&mut e)?;
        e.retain(|c| c != ' ');
        e = e.replace("[a]", &("(".to_owned() + &l.to_string() + ")"));
        let e = {
            while e.contains("--") || e.contains("-+") || e.contains("+-") || e.contains("++") {
                e = e.replace("+-", "-");
                e = e.replace("-+", "-");
                e = e.replace("--", "+");
                e = e.replace("++", "+");
            }
            e.trim()
        };
        match e {
            "q" => break,
            "h" => help(),
            "d" => println!("≈ {}", fraction_to_float(l)),
            "c" => credits(),
            _ => {
                let ans = calculate(e);
                match ans {
                    Err(e) => println!("Error: {:?}", e),
                    Ok(o) => {
                        l = o;
                        println!("= {}", o)
                    },
                }
            }
        }
    }
    Ok(())
}
