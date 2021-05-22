use std::collections::HashMap;
use std::io;
use std::process;

fn main() {
    let (n, ppc, avc) = read_input();
    let result = max_advertisement(n, ppc, avc);

    println!("{}", result);
}

fn max_advertisement(n: u32, profit_per_click: Vec<i64>, average_clicks: Vec<i64>) -> i64 {
    let mut visited_indices = HashMap::new();
    let mut max_amount = 0;

    // go through 1 to n
    for _i in 0..n {
        let mut max_a = std::i64::MIN;
        let mut max_b = std::i64::MIN;

        // track the index of max value in array
        let mut temp_index_a = -1;
        let mut temp_index_b = -1;

        for ai in 0..n {
            let iter_ai = ai as usize;
            let key_a = format!("a{}", iter_ai);
            // select a[i] which is max from profit_per_click
            if profit_per_click[iter_ai] > max_a && visited_indices.get(&key_a) != Some(&1) {
                max_a = profit_per_click[iter_ai];
                temp_index_a = iter_ai as i32;
            }

            let iter_bi = ai as usize;
            let key_b = format!("b{}", iter_bi);
            // select b[i] which is max from average_clicks
            if average_clicks[iter_bi] > max_b && visited_indices.get(&key_b) != Some(&1) {
                max_b = average_clicks[iter_bi];
                // update the temp_index to mark this index as visited
                temp_index_b = iter_bi as i32;
            }
        }

        // mark this index as visited
        if temp_index_a != -1 {
            visited_indices.insert(format!("a{}", temp_index_a), 1);
        }

        // mark the index as visited
        if temp_index_b != -1 {
            visited_indices.insert(format!("b{}", temp_index_b), 1);
        }

        // calculate the revenue from selected pair of a[i] & b[i]
        if max_a != std::i64::MIN && max_b != std::i64::MIN {
            let temp_value = max_a * max_b;
            max_amount += temp_value;
        }
    }

    max_amount
}

fn read_input() -> (u32, Vec<i64>, Vec<i64>) {
    let n = read_number();
    let ppc = read_numbers(n);
    let avc = read_numbers(n);

    (n, ppc, avc)
}

fn read_numbers(n: u32) -> Vec<i64> {
    let mut input = String::new();
    let mut nums: Vec<i64> = Vec::new();
    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let words = input.split_whitespace();

    for word in words.enumerate() {
        let num: i64 = (word.1).parse().unwrap();
        nums.push(num);
    }

    if nums.len() < n as usize || nums.len() > n as usize {
        panic!("Please enter {} numbers separated by space", n);
    }

    nums
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
