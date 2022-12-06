#[derive(Debug, Clone)]
struct Boat {
    lines: Vec<Vec<char>>,
    instructions: Vec<Vec<u32>>
}

fn parse(input: &str) -> Boat {
    let lines = input.lines();

    let mut index = 0;
    let mut boat = Boat { 
        lines: Vec::new(),
        instructions: Vec::new()
    };

    let mut instructions = false;

    let mut instruction_buffer = Vec::new();
    for line in lines {
        let chars = line.chars();
        let is_division = chars.count() == 0;

        if is_division {
            instructions = true;
            continue;
        }

        if instructions {
            let mut buffer = String::new();
            for char in line.chars() {
                if char.is_numeric() {
                    buffer.push(char);
                } else if buffer.len() > 0 {
                    instruction_buffer.push(buffer.parse::<u32>().unwrap());
                    buffer = String::new();
                }
            }

            if buffer.len() > 0 {
                instruction_buffer.push(buffer.parse().unwrap());
            }


            boat.instructions.push(instruction_buffer.clone());
            instruction_buffer.clear();

        } else {
            let mut cursor = 0;
            for char in line.chars() {
                if index == boat.lines.len() {
                    boat.lines.insert(index, Vec::new());
                }

                if char.is_alphabetic() {
                    if let Some(line) = boat.lines.get_mut(index) {
                        line.insert(0, char);
                    }
                }

                cursor += 1;

                if cursor == 4 {
                    index += 1;
                    cursor = 0;
                }
            }

            index = 0;
        }
    }

    boat
}

fn arrange(boat: &mut Boat, mantain_order: bool) {
    for x in &boat.instructions {
        let quantity = x[0];
        let from = x[1] - 1;
        let to = x[2] - 1;

        if let Some(from) = boat.lines.get_mut(from as usize) {
            let mut buffer = Vec::with_capacity(quantity as usize);
            for _ in 0..quantity {
                if let Some(item) = from.pop() {
                    if mantain_order {
                        buffer.insert(0, item);
                    } else {
                        buffer.push(item);
                    }
                }
            }

            if let Some(to) = boat.lines.get_mut(to as usize) {
                to.extend(buffer);
            }
        }

    }
}

fn get_heads(boat: &Boat) -> String {
    // print heads
    let mut heads = String::new();
    for line in &boat.lines {
        if let Some(head) = line.last() {
            heads.push(*head);
        }
    }

    heads
}

pub fn run(input: String) {
    let mut boat = parse(&input);
    let mut boat2 = boat.clone();

    arrange(&mut boat, false);
    arrange(&mut boat2, true);

    println!("Day 5:");
    println!("  Part 1: {}", get_heads(&boat));
    println!("  Part 2: {}", get_heads(&boat2));
}
