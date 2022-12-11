use std::collections::HashMap;

const TOTAL_SPACE: u64 = 70000000;
const REQUIRED_SPACE: u64 = 30000000;

fn find_candidate_to_free_space(folders: &HashMap<String, u64>) -> u64 {
    let total_used = folders.get("/").unwrap();
    let space_left = TOTAL_SPACE - total_used;

    *folders
        .values()
        .filter(|v| space_left + **v >= REQUIRED_SPACE)
        .min()
        .unwrap()
}

pub fn run(input: String) {
    let mut route = Vec::new();

    let mut folders = HashMap::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        match split.as_slice() {
            ["$", "cd", to] => {
                let is_back = to == &"..";
                if is_back {
                    route.pop();
                } else {
                    route.push(to.trim().to_string());
                }
            }
            [size, _] if size.chars().all(|x| x.is_numeric()) => {
                let size = size.parse::<u64>().unwrap();

                for index in 0..route.len() {
                    let path = route[0..=index].join("/");
                    let entry = folders.entry(path).or_insert(0);
                    *entry += size;
                }
            }
            _ => {}
        }
    }

    let sum = folders.values().filter(|v| v <= &&100000).sum::<u64>();

    let to_free = find_candidate_to_free_space(&folders);

    println!("Day 7:");
    println!("  Part 1: {}", sum);
    println!("  Part 2: {}", to_free);
}
