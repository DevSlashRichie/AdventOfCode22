use std::collections::HashSet;

fn find_marker(input: &str, size: usize) -> u32 {
    for index in 0..input.chars().count() {
        if index >= size {
            let mut buff = HashSet::new();
            let from = index - size;
            let sub = &input[from..index];
            buff.extend(sub.chars());

            if buff.len() == size {
                return index as u32;
            }
        }
    }

    panic!("No marker found");
}

pub fn run(input: String) {
    let marker = find_marker(&input, 4);
    let message = find_marker(&input, 14);

    println!("Day 6");
    println!("  Part 1: {}", marker);
    println!("  Part 2: {}", message);
}
