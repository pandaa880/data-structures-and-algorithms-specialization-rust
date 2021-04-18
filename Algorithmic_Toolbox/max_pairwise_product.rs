use std::io;
use std::process;


fn main() {
    let n: u64 = read_length();
    let nums: Vec<u64> = read_numbers(n);

    let product: u64 = maximum_pairwise_product(nums);

    println!("{}", product);
}

fn maximum_pairwise_product(numbers: Vec<u64>) -> u64 {
    let mut max1 = 0;
    let mut i1 = 0;

    for (index, x) in numbers.iter().enumerate() {
        if max1 == 0 && index == 0 {
            max1 = *x;
        }

        if x > &max1 {
            max1 = *x;
            i1 = index;
        }
    }

    let mut max2 = 0;
    
    for (index, x) in numbers.iter().enumerate() {
        if x > &max2 && index != i1 {
            max2 = *x;
        }
    }

    max1 * max2
}

fn read_length() -> u64 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let digit: u64;

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

fn read_numbers(n: u64) -> Vec<u64> {
    let mut input = String::new();
    let mut count: u64 = n;

    io::stdin().read_line(&mut input).unwrap();

    let mut numbers = Vec::new();

    for num in input.trim().split_whitespace() {
        if count == 0 {
            panic!("Please enter numbers till the range you have provided");
        }
        match num.parse::<u64>() {
            Ok(val) => {
                numbers.push(val);
                count = count - 1;
            },
            Err(_err) => {
                println!("Please enter valid number");
                process::exit(1);
            }
        }
    }

    numbers
}
