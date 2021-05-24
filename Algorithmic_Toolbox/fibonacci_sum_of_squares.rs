use std::io;
use std::process;

// the formula to compute sum of squares of nth fibonacci numbers
// F(n)*F(n+1)

fn main() {
    let a: u64 = read_number();
    // based on formula we will calculate n+1 fibonacci numbers last digit
    let fibonacci_num_n: u128 = get_fibonacci_huge(a, 10);
    let fibonacci_num_n_plus_one: u128 = get_fibonacci_huge(a + 1, 10);

    let result = (fibonacci_num_n * fibonacci_num_n_plus_one) % 10;

    println!("{}", result);
}

fn get_pisano_period(m: u64) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    let mut res: u64 = 0;

    // based on theory that period will not exceed m*m
    for i in 0..m * m {
        let temp;
        temp = curr;
        curr = (prev + curr) % m;
        prev = temp;

        if prev == 0 && curr == 1 {
            res = i + 1
        }
    }

    res
}

fn get_fibonacci_huge(n: u64, m: u64) -> u128 {
    let period_length: u64 = get_pisano_period(m);
    let n = n % period_length;

    let mut r = n;

    let mut prev = 0;
    let mut curr = 1;

    for _i in 1..n {
        r = (prev + curr) % m;
        prev = curr;
        curr = r
    }

    let res: u128 = (r % m).into();

    res
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
            println!("Please enter valid length number.");
            process::exit(1);
        }
    }

    digit
}
