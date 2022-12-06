use std::ops::{RangeInclusive};


#[derive(Debug)]
struct Pair {
    range_a: RangeInclusive<i32>,
    range_b: RangeInclusive<i32>,
}

impl Pair {

    fn intersect(&self) -> bool {
        self.range_b.start() >= self.range_a.start() && self.range_b.end() <= self.range_a.end()
        || self.range_a.start() >= self.range_b.start() && self.range_a.end() <= self.range_b.end()
    }

    fn overlap(&self) -> bool {
        self.range_a.start() <= self.range_b.start() && self.range_a.end() >= self.range_b.start()
        || self.range_b.start() <= self.range_a.start() && self.range_b.end() >= self.range_a.start()
    }
    

}

fn parse_range(pair: &str) -> RangeInclusive<i32> {
    let mut parts = pair.split('-');

    let start = parts.next().unwrap();
    let end = parts.next().unwrap();

    let start = start.parse::<i32>().unwrap();
    let end = end.parse::<i32>().unwrap();

    start..=end
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(",");
            Pair {
                range_a: parse_range(parts.next().unwrap()),
                range_b: parse_range(parts.next().unwrap()),
            }
        })
        .collect()
}

pub fn run(input: String) {
    let pairs = parse_pairs(&input);

    let intersect = pairs.iter()
        .filter(|pair| pair.intersect())
        .count();

    println!("Day 4:");
    println!("  {}", intersect);

    let overlap = pairs.iter()
        .filter(|pair| pair.overlap())
        .count();
    
    println!("  {}", overlap);
}