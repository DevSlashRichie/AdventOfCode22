use std::collections::{HashSet, HashMap};

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone)]
struct Rucksack {
    first: String,
    second: String,
}

impl Rucksack {
    fn total(&self) -> String {
        self.first.clone() + &self.second
    }
}


fn prepare_rucksacks(input: &str) -> Vec<Rucksack> {
    input.lines()
        .map(|line| {
            let half = line.len() / 2;
            Rucksack {
                first: line[..half].to_string(),
                second: line[half..].to_string(),
            }
        })
        .collect()
}

fn get_letter_position(letter: &char) -> usize {
    let letter = letter.to_ascii_lowercase();
    ABC.chars().position(|c| c == letter).unwrap()
}

fn get_weight(letter: char) -> usize {
    let s = get_letter_position(&letter) + 1;
    if letter.is_uppercase() {
        s + ABC.len()
    } else {
        s
    }
}

pub fn run(input: String) {
    let rucksacks = prepare_rucksacks(&input);

    let repeated_letters = rucksacks.iter()
        .filter_map(|r| {
            let set = r.first.chars().collect::<HashSet<_>>();

            let repeated = r.second.chars()
                .filter(|c| set.contains(c))
                .collect::<Vec<_>>();
            
            repeated.first().copied()
        })
        .collect::<Vec<_>>();

    let weight = repeated_letters.iter()
        .map(|c| {
            get_weight(*c)
        })
        .sum::<usize>();

    println!("Day 3:");
    println!("  Part 1: {}", weight);

    // Part 2

    let groups = rucksacks.chunks(3);
    let mut total = 0;
    for group in groups {
        let first = group[0].total().chars().collect::<HashSet<_>>();

        let repeated = &group[1..].iter()
            .fold(first, |acc, x| {
                acc.intersection(&x.total().chars().collect::<HashSet<_>>())
                    .copied()
                    .collect::<HashSet<_>>()
            })
            .into_iter()
            .map(|c| get_weight(c))
            .sum::<usize>();

        total += repeated;
    }


        println!("  Part 2: {}", total);
}