mod common;

mod day_1;
mod day_2;

use std::fs;

fn read_data(day: &str) -> String {
    let f = format!("data/day_{}.txt", day);
    fs::read_to_string(f).unwrap()
}

fn main() {
    day_1::run_1(read_data("1"));
    day_2::run(read_data("2"));
}
