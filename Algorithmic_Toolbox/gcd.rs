use std::io;

fn main() {
    let nums: [u128; 2] = read_numbers();

    let [a, b] = nums;

    let result = gcd(a, b);

    println!("{}", result);
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }

    let r: u128 = a % b;

    return gcd(b, r);
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
