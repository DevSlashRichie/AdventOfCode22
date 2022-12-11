use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Direction {
    fn steps(&self) -> u32 {
        match self {
            Direction::Up(steps) => *steps,
            Direction::Down(steps) => *steps,
            Direction::Left(steps) => *steps,
            Direction::Right(steps) => *steps,
        }
    }
}

fn extract_number(number: &[char]) -> u32 {
    number.iter().collect::<String>().parse::<u32>().unwrap()
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        let value = value.chars().collect::<Vec<_>>();

        let value = value.as_slice();

        match value {
            ['R', ' ', number @ ..] => Direction::Right(extract_number(number)),
            ['L', ' ', number @ ..] => Direction::Left(extract_number(number)),
            ['U', ' ', number @ ..] => Direction::Up(extract_number(number)),
            ['D', ' ', number @ ..] => Direction::Down(extract_number(number)),
            _ => panic!("Invalid command: {:?}", value),
        }
    }
}

struct Rope {
    head: Knot,
    tail: Vec<Knot>,
    visited: HashSet<(i32, i32)>,
}

#[derive(Debug)]
struct Knot(i32, i32);

impl Knot {
    fn new() -> Self {
        Self(0, 0)
    }

    fn move_once(&mut self, direction: &Direction) {
        match direction {
            Direction::Up(_) => self.0 += 1,
            Direction::Down(_) => self.0 -= 1,
            Direction::Left(_) => self.1 -= 1,
            Direction::Right(_) => self.1 += 1,
        }
    }

    fn into_tuple(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    fn adjust_position(&self, reference: &Knot) -> Option<Knot> {
        let (x1, y1) = self.into_tuple();
        let (x2, y2) = reference.into_tuple();

        let x = (x2 as f64 - x1 as f64).abs();
        let y = (y2 as f64 - y1 as f64).abs();

        let new_x = if x1 < x2 { x2 - 1 } else { x2 + 1 };

        let new_y = if y1 < y2 { y2 - 1 } else { y2 + 1 };

        match (x, y) {
            (x, y) if x >= 2.0 && y >= 2.0 => Some(Knot(new_x, new_y)),
            (x, _) if x >= 2.0 => Some(Knot(new_x, y2)),
            (_, y) if y >= 2.0 => Some(Knot(x2, new_y)),
            _ => None,
        }
    }
}

impl Rope {
    fn new(size: u8) -> Self {
        if size == 0 {
            panic!("Rope size must be greater than 0");
        }

        let visited = HashSet::from_iter([(0, 0)]);
        let tail = (0..size).into_iter().map(|_| Knot::new()).collect();

        Rope {
            visited,
            head: Knot::new(),
            tail,
        }
    }

    fn last_body(&self) -> &Knot {
        self.tail.last().unwrap()
    }

    fn trasverse_commands(&mut self, commands: &Vec<Direction>) {
        for command in commands {
            self.move_rope(command);
        }
    }

    fn move_rope(&mut self, direction: &Direction) {
        let steps = direction.steps();
        for _ in 0..steps {
            self.head.move_once(direction);

            for i in 0..self.tail.len() {
                let updated_position = {
                    let before = if i > 0 { &self.tail[i - 1] } else { &self.head };

                    self.tail[i].adjust_position(before)
                };

                if let Some(pos) = updated_position {
                    self.tail[i] = pos;
                }
            }

            self.visited.insert(self.last_body().into_tuple());
        }
    }
}

pub fn run(input: String) {
    let commands = input
        .lines()
        .into_iter()
        .map(Direction::from)
        .collect::<Vec<_>>();

    let mut part_1 = Rope::new(1);
    part_1.trasverse_commands(&commands);

    let mut part_2 = Rope::new(9);
    part_2.trasverse_commands(&commands);

    println!("Day 9:");
    println!("  Part 1: {}", part_1.visited.len());
    println!("  Part 2: {}", part_2.visited.len());
}
