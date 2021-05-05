use std::io;
use std::process;

fn main() {
    let n: usize = read_length();

    let result = fibonacci(n);

    println!("{}", result);
}

fn fibonacci(n: usize) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();
    numbers.push(0);
    numbers.push(1);
    
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    for num in 2..(n + 1) {
        let next_num: u32 = numbers[&num - 1] + numbers[&num - 2];
        numbers.push(next_num)
    }

    numbers[n]
}

fn read_length() -> usize {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let digit: usize;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }
        Err(_err) => {
            println!("Please enter valid length number.");
            process::exit(1);
        }
    }

    digit
}
