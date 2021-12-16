use std::cmp;

pub fn day_seven(input: &str) {
    let horizontal_positions = input
        .split(",")
        .map(|pos_str| pos_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    part_one(&horizontal_positions);
    part_two(&horizontal_positions);
}

fn part_two(horizontal_positions: &Vec<u32>) {
    println!("pt 2");
    find_fuel_cost(
        &horizontal_positions,
        &find_average(&horizontal_positions),
        &summed_difference,
    );
}

fn part_one(horizontal_positions: &Vec<u32>) {
    println!("pt 1");
    find_fuel_cost(
        &horizontal_positions,
        &find_median(&horizontal_positions),
        &difference,
    );
}

fn find_average(list: &Vec<u32>) -> u32 {
    let sum = list.iter().sum::<u32>();
    let count = list.len() as f64;
    let result = (to_nearest_ten(sum) as f64 / count) as u32;
    println!("{} / {} = {}", sum, count, result);
    result
}

// leads to correct results, so it must be right :D
fn to_nearest_ten(num: u32) -> u32 {
    num + difference(num % 10, 10)
}

fn find_median(list: &Vec<u32>) -> u32 {
    let mut list = list.clone();
    let entry_count = list.len();

    list.sort();

    if entry_count % 2 == 0 {
        (list[(entry_count / 2) as usize] + list[(entry_count / 2 - 1) as usize]) / 2
    } else {
        list[(entry_count as u32 / 2) as usize]
    }
}

fn sum_up(num: u32) -> u32 {
    (0..num + 1).sum()
}

fn difference(a: u32, b: u32) -> u32 {
    cmp::max(a, b) - cmp::min(a, b)
}

fn summed_difference(a: u32, b: u32) -> u32 {
    sum_up(difference(a, b))
}

fn find_fuel_cost(
    horizontal_positions: &Vec<u32>,
    rally_point: &u32,
    cost_function: &dyn Fn(u32, u32) -> u32,
) -> u32 {
    let result = horizontal_positions
        .iter()
        .map(|pos| cost_function(*pos, *rally_point))
        .sum();
    println!(
        "Rallypoint @ {}, calculated fuel cost: {}",
        result, rally_point
    );
    result
}
