mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use std::fs;

fn read_data(day: u8) -> String {
    let f = format!("data/day_{}.txt", day);
    fs::read_to_string(f).unwrap()
}

fn main() {
    day_1::run_1(read_data(1));
    day_2::run(read_data(2));
    day_3::run(read_data(3));
    day_4::run(read_data(4));
    day_5::run(read_data(5));
    day_6::run(read_data(6));
    day_7::run(read_data(7));
    day_8::run(read_data(8));
}
