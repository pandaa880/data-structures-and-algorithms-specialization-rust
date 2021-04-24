use std::io;
use std::process;

fn main() {
    let n: u128 = read_length();
    println!("{}", n);
}

fn read_length() -> u128 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let digit: u128;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }
        Err(_err) => {
            println!("Please enter valid number.");
            process::exit(1);
        }
    }

    digit
}
