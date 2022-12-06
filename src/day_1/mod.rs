fn split_by_line(s: &str) -> Vec<&str> {
    s
        .split("\n")
        .collect::<Vec<&str>>()
}

fn calculate_elfs(s: &Vec<&str>) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut cursor = 0;

    for x in s {
        if x == &"" {
            numbers.push(cursor);
            cursor = 0;
        } else {
            if let Ok(n) = x.parse::<i32>() {
                cursor += n;
            }
        }
    }

    numbers.push(cursor);
    numbers
}

pub fn run_1(input: String) {
    let elfs = split_by_line(&input);

    let mut elfs_amounts = calculate_elfs(&elfs);

    elfs_amounts.sort();
    
    let maximum = elfs_amounts.last();

    println!("Day 1: ");

    if let Some(maximum) = maximum {
        println!("  Part 1: {}", maximum);
    } 

    // take last 3 
    let last_3 = elfs_amounts.split_off(elfs_amounts.len() - 3);
    let total = last_3.iter().sum::<i32>();
    println!("  Part 2: {}", total);
}