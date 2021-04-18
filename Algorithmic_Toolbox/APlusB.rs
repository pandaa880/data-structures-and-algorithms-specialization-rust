use std::io;

fn main() {
    let mut input = String::new();

    // read the inputs
    io::stdin().read_line(&mut input).expect("User input failed");

    // separate the two numbers
    let mut words = input.split_whitespace();
    
    // convert to numbers
    let num1: i64 = words.next().unwrap().parse().unwrap();
    let num2: i64 = words.next().unwrap().parse().unwrap();

    println!("{}", num1 + num2);
}
