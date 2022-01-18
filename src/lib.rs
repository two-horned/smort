pub mod syntax;
pub mod calculator;

pub fn help() {
    println!("Available Commands:");
    println!("    'h' / 'help': show help,");
    println!("    'q' / 'quit': quit application,");
    println!("");
    println!("Available Operants:");
    println!("    + (addition),");
    println!("    - (subtraction),");
    println!("    * (multiplication),");
    println!("    / (division),");
    println!("    ^ (power of),");
    println!("    % (modulo),");
    println!("You can use brackets too, but only round ones.");
}

pub fn is_quit(e: &str) -> bool {
    match e {
        "q"|"quit" => true,
        _ => false,
    }
}

pub fn is_help(e: &str) -> bool {
    match e {
        "h"|"help" => true,
        _ => false,
    }
}
