
use std::io;
use std::process;

fn main() {
    let (d, m, n, stops): (u64, u64, u64, Vec<u64>) = read_input();

    let r = min_refills(d, m, n, &stops);

    println!("{}", r);
}

fn min_refills(_d: u64, m: u64, n: u64, stops: &Vec<u64>) -> i32 {
    let mut num_refill: i32 = 0;
    let mut current_refill: u64 = 0;

    // start from starting point
    while current_refill <= n {
        let last_refill = current_refill;

        // find optimal point to refill
        while current_refill <= n
            && (stops[(current_refill + 1) as usize] - stops[last_refill as usize]) <= m
        {
            current_refill += 1;
        }

        // if can't move forward then return impossible
        if current_refill == last_refill {
            return -1;
        }

        // increment number of refills
        if current_refill <= n {
            num_refill += 1;
        }
    }
    num_refill
}

fn read_input() -> (u64, u64, u64, Vec<u64>) {
    let d = read_number();
    let m = read_number();
    let n = read_number();
    let stops: Vec<u64> = read_numbers(d);

    (d, m, n, stops)
}

fn read_numbers(d: u64) -> Vec<u64> {
    let mut input = String::new();
    let mut stops: Vec<u64> = Vec::new();

    // starting point
    stops.push(0);

    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let words = input.split_whitespace();

    for num in words.enumerate() {
        let t: u64 = (num.1).parse().unwrap();
        stops.push(t);
    }

    // ending point
    stops.push(d);

    stops
}

fn read_number() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let digit: u64;

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