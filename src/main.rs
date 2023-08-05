use smort::fraction::Fraction;

//use std::f64::consts::{PI, E};
use {
    smort::calculator::{calculate,fraction_to_float},
    std::io::{self, Write},
};

fn credits() {
    println!("Smort Calculator by Said Kadrioski");
}

fn help() {
    println!("Available Commands:");
    println!("    'h' or 'help': show help,");
    println!("    'd' or 'decimal': show last result as decimal,");
    println!("    '[a]': apply following operation on the last result,");
    println!("    'c' or 'credits': show credits,");
    println!("    'q' or 'quit': quit application,");
    println!("    'l' or 'clear': to clear the screen.");
    println!();
    println!("Available Operants:");
    println!("    + (addition),");
    println!("    - (subtraction),");
    println!("    * (multiplication),");
    println!("    / (division),");
    println!("    ^ (power of),");
    println!("    % (modulo),");
    println!("    ! (faculty)");
    println!("You can use parenthesis too.");
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> io::Result<()> {
    clear();
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
            "q" | "quit" => break,
            "h" | "help" => help(),
            "d" | "decimal" => println!("≈ {}", fraction_to_float(l)),
            "c" | "credits" => credits(),
            "l" | "clear" => clear(),
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
