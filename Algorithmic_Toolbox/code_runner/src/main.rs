use std::convert::TryInto;
use std::io;

fn main() {
    let nums: [u128; 2] = read_numbers();

    let [_a, b] = nums;

    let period_length: u128 = get_pisano_period(b);

    println!("{}", period_length);
}

fn get_pisano_period(m: u128) -> u128 {
    let mut curr: u32 = 0;
    let mut next: u32 = 1;
    let mut r: u32 = curr + next;
    let mut count: u32 = 0;
    let max_limit: u32 = m.try_into().unwrap();
    let mut result = 2;

    // based on thoery that period of Fi mod m does not exceed m^2
    while count < max_limit * max_limit {
        r = (curr + next) % max_limit;
        curr = next;
        next = r;

        if curr == 0 && next == 1 {
            result = (count + 1).try_into().unwrap()
        }

        count += 1
    }

    result
}

fn get_fibonacci_huge(n: u128, m: u128) {
    // TODO: finish this function
}

fn read_numbers() -> [u128; 2] {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let mut words = input.split_whitespace();

    // convert to numbers
    let num1: u128 = words.next().unwrap().parse().unwrap();
    let num2: u128 = words.next().unwrap().parse().unwrap();

    [num1, num2]
}
