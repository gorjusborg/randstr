use std::env;
use std::process;

use randstr::ascii_lowercase_chars;
use randstr::ascii_numeric_chars;
use randstr::ascii_uppercase_chars;
use randstr::randstr;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    let prog = args.remove(0);

    if args.len() < 1 {
        eprintln!("Usage: {} length", prog);
        process::exit(1);
    }

    let length = args.first().unwrap();

    let length = length.parse::<usize>().unwrap_or_else(|_e| {
        eprintln!("Error: provided length '{}' is not a number", length);
        process::exit(1);
    });

    let mut choices = Vec::new();
    choices.extend(ascii_lowercase_chars());
    choices.extend(ascii_uppercase_chars());
    choices.extend(ascii_numeric_chars());

    println!("{}", randstr(&choices, length));
}
