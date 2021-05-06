use std::io;

fn main() {
    let nums: (u64, u64) = read_numbers();

    let (a, b) = nums;

    let result: u128 = get_fibonacci_huge(a, b);

    println!("{}", result);
}

fn get_pisano_period(m: u64) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    let mut res: u64 = 0;

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

    let mut prev = 0;
    let mut curr = 1;

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    for _i in 0..n - 1 {
        let temp;
        temp = curr;
        curr = (prev + curr) % m;
        prev = temp;
    }

    let r: u128 = (curr % m).into();

    r
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
