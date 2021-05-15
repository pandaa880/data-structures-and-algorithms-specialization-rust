use std::io;
use std::process;

fn main() {
    let m: u32 = read_number();

    println!("{}", m);
}

fn _read_numbers() -> (u64, u64) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let mut words = input.split_whitespace();

    // convert to numbers
    let num1: u64 = words.next().unwrap().parse().unwrap();
    let num2: u64 = words.next().unwrap().parse().unwrap();

    (num1, num2)
}

fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let digit: u32;

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
