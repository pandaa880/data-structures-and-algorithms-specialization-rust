use std::io;
use std::process;

fn main() {
    let m: u32 = read_number();

    let number_of_coins = change(m);

    println!("{}", number_of_coins);
}

fn change(mut m: u32) -> u32 {
    let mut coins = 0;

    if m >= 10 {
        coins = coins + (m / 10);
        m = m % 10;
    }
    if m >= 5 {
        coins = coins + (m / 5);
        m = m % 5;
    }

    if m >= 1 {
        coins = coins + m;
    }

    coins
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
