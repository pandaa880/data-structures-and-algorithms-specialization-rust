use std::io;
use std::process;

fn main() {
    let n: usize = read_length();
    let mut numbers: Vec<u32> = Vec::new();
    numbers.push(0);
    numbers.push(1);

    let result = fibonacci(n, numbers);

    println!("{}", result);
}

fn fibonacci(n: usize, mut arr: Vec<u32>) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    for num in 2..(n + 1) {
        let mut next_num: u32 = arr[&num - 1] + arr[&num - 2];
        // if number is 2 digit then only store the last digit
        if next_num >= 10 {
            next_num = next_num % 10;
        }

        arr.push(next_num)
    }

    arr[n]
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
