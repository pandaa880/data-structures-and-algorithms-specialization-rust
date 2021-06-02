use std::io;
use std::process;

fn main() {
    let n = read_number();
    let segments = read_input(n);
    let sorted_segments = sort_segments(segments.clone());
    let result = collect_signatures(sorted_segments);

    println!("{}", result.len());
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
}

fn collect_signatures(segments: Vec<(u64, u64)>) -> Vec<u64> {
    let mut min_steps = Vec::new();

    let mut point = segments[0].1;
    min_steps.push(point);

    for i in 0..segments.len() {
        if point < segments[i].0 || point > segments[i].1 {
            point = segments[i].1;
            min_steps.push(point)
        }
    }

    min_steps
}

fn sort_segments(mut segments: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    for i in 0..segments.len() {
        for j in 0..segments.len() - i - 1 {
            if segments[j].1 > segments[j + 1].1 {
                let temp = segments[j + 1];
                segments[j + 1] = segments[j];
                segments[j] = temp;
            }
        }
    }

    segments
}

fn read_input(n: u32) -> Vec<(u64, u64)> {
    let mut segments = Vec::new();

    for _i in 0..n {
        let points = read_numbers();
        segments.push(points);
    }

    segments
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

fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let digit: u32;

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