use std::io;

// the formula to compute sum of nth fibonacci number is
// F(n+2) - 1

fn main() {
    let (a, b): (u64, u64) = read_numbers();
    // based on formula we will calculate n+2 fibonacci numbers last digit
    let fibonacci_num_a: u128 = get_fibonacci_huge(a + 1, 10);
    let fibonacci_num_b: u128 = get_fibonacci_huge(b + 2, 10);

    let result_a;
    let result_b;

    if fibonacci_num_a <= 0 {
        result_a = (10 + fibonacci_num_a) - 1;
    } else {
        result_a = fibonacci_num_a - 1
    }

    if fibonacci_num_b <= 0 {
        result_b = (10 + fibonacci_num_b) - 1;
    } else {
        result_b = fibonacci_num_b - 1 + 10;
    }

    let r = (result_b - result_a) % 10;

    println!("{}", r);
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

fn read_numbers() -> (u64, u64) {
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
