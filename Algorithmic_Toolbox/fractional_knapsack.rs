use std::cmp;
use std::io;
use std::process;

fn main() {
    let (n, w, items) = read_input();

    let (total_value, _total_weight) = fractional_knapsack(n, w, items);

    println!("{:0.4}", total_value);
}

fn fractional_knapsack(n: u64, w: u64, mut items: Vec<[u64; 3]>) -> (f64, f64) {
    let mut bag_capacity = w;
    let mut total_value: f64 = 0.0;
    let mut total_weight: f64 = 0.0;

    for _i in 0..n {
        // set variable to get max unit value for each iteration
        let mut max_per_unit_value: f64 = 0.0;
        // get the index of max unit value for each iteration
        let mut max_unit_index = 0;
        // temporary index holder to update the value later
        let mut index = 0;

        for item in &mut items {
            // compute per unit value
            if item[2] == 1 {
                continue;
            }
            let unit_value: f64 = item[0] as f64 / item[1] as f64;

            // compare the unit value with previous max and make sure we are not comparing with already visited items
            // item[2] acts as wether this item has been visited before or not
            if unit_value > max_per_unit_value {
                max_per_unit_value = unit_value;
                max_unit_index = index;
            }

            index += 1;
        }

        // once we get max unit value for particular iteration mark the item on the index as visited
        items[max_unit_index][2] = 1;
        // get the weight of item which has max per unit value
        let weight = items[max_unit_index][1];

        // fill the bag with whole item or max possible
        let a = cmp::min(weight, bag_capacity);

        // calculate total values of filled item
        total_weight += a as f64;
        total_value += a as f64 * max_per_unit_value;

        // update weight
        items[max_unit_index][1] -= a;
        bag_capacity -= a;
    }

    (total_value, total_weight)
}

fn read_input() -> (u64, u64, Vec<[u64; 3]>) {
    let mut r: Vec<[u64; 3]> = Vec::new();

    let (n, w) = read_numbers();
    for _i in 0..n {
        let (a, b) = read_numbers();
        r.push([a, b, 0])
    }

    (n, w, r)
}

// reads two space separated numbers and return them as tuple
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