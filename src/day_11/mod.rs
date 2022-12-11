use std::{collections::VecDeque, thread::current};

#[derive(Debug, Default, Clone)]
struct TestResult {
    is_true: usize,
    is_false: usize,
}

#[derive(Debug, Clone)]
enum OperationType {
    Add(i64, i64),
    Multiply(i64, i64),
    Divide(i64, i64),
    Subtract(i64, i64),
}

impl Default for OperationType {
    fn default() -> Self {
        OperationType::Add(0, 0)
    }
}

fn eval_number(a: i64, with: u64) -> u64 {
    if a < 0 {
        with
    } else {
        a.try_into().unwrap()
    }
}

fn execute_operation(t: (i64, i64, u64), f: impl FnOnce(u64, u64) -> u64) -> u64 {
    let with = t.2;
    let a = eval_number(t.0, with);
    let b = eval_number(t.1, with);
    f(a, b)
}

impl OperationType {
    fn execute(&self, old: u64) -> u64 {
        match self {
            OperationType::Add(a, b) => execute_operation((*a, *b, old), |a, b| a + b),
            OperationType::Multiply(a, b) => execute_operation((*a, *b, old), |a, b| a * b),
            OperationType::Divide(a, b) => execute_operation((*a, *b, old), |a, b| a / b),
            OperationType::Subtract(a, b) => execute_operation((*a, *b, old), |a, b| a - b),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: OperationType,
    test: u8,
    test_result: TestResult,
    inspection_amount: u64,
}

impl Monkey {
    fn get_monkey_destination(&self, worry_level: u64) -> usize {
        if &worry_level % self.test as u64 == 0 {
            self.test_result.is_true
        } else {
            self.test_result.is_false
        }
    }

    fn inspect_next(&mut self) -> Option<u64> {
        let next = self.items.pop_front();
        if next.is_some() {
            self.inspection_amount += 1;
        }
        next
    }
}

fn parse_last_char(line: &str) -> u8 {
    let s = line
        .chars()
        .rev()
        .take_while(|x| x.is_numeric())
        .fold(String::new(), |mut acc, s| {
            acc.insert(0, s);
            acc
        });

    s.parse().unwrap()
}

fn parse_block(block: &Vec<String>) -> Monkey {
    if block.len() < 6 {
        panic!("Block too short");
    }

    let mut monkey = Monkey::default();

    for (index, line) in block.into_iter().skip(1).enumerate() {
        let line = line.trim();
        match index {
            0 => {
                let (_, numbers) = line.split_once(":").unwrap();
                let items = numbers
                    .split(",")
                    .map(|x| x.trim().parse().unwrap())
                    .collect();
                monkey.items = items;
            }
            1 => {
                let (_, operation) = line.split_once("=").unwrap();
                let operation = operation.trim();

                let mut a = String::new();
                let mut b = String::new();
                let mut sign = String::new();

                for c in operation.chars() {
                    if c.is_whitespace() {
                        continue;
                    }

                    if sign.is_empty() {
                        a.push(c);
                    } else {
                        b.push(c);
                    }

                    if c == '+' || c == '-' || c == '*' || c == '/' {
                        sign.push(c);
                    }
                }

                let parse = |s: String| s.parse().unwrap_or(-1);

                monkey.operation = match sign.as_str() {
                    "+" => OperationType::Add(parse(a), parse(b)),
                    "-" => OperationType::Subtract(parse(a), parse(b)),
                    "*" => OperationType::Multiply(parse(a), parse(b)),
                    "/" => OperationType::Divide(parse(a), parse(b)),
                    _ => panic!("Unknown operation: {}", sign),
                };
            }
            2 => monkey.test = parse_last_char(line),
            3 => monkey.test_result.is_true = parse_last_char(line) as usize,
            4 => monkey.test_result.is_false = parse_last_char(line) as usize,
            _ => {}
        }
    }

    monkey
}

fn parse(input: &str) -> Vec<Monkey> {
    let lines = input.lines();

    let mut current_block = Vec::new();
    let mut monkeys = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        current_block.push(line.to_string());

        if current_block.len() == 6 {
            let monkey = parse_block(&current_block);
            monkeys.push(monkey);
            current_block.clear();
        }
    }

    monkeys
}

struct Observer<T>
where
    T: Fn(u64) -> u64,
{
    rounds: usize,
    relif: T,
}

impl<T> Observer<T>
where
    T: Fn(u64) -> u64,
{
    fn new(rounds: usize, relif: T) -> Self {
        Self { rounds, relif }
    }

    fn observe(&self, mut monkeys: Vec<Monkey>) -> u64 {
        for _ in 0..self.rounds {
            for monkey_index in 0..monkeys.len() {
                while let Some(prev) = monkeys[monkey_index].inspect_next() {
                    let monkey = &monkeys[monkey_index];

                    let worry_level = monkey.operation.execute(prev);
                    let worry_level = (self.relif)(worry_level);
                    let next_monkey = monkey.get_monkey_destination(worry_level);

                    monkeys[next_monkey].items.push_back(worry_level);
                }
                // println!("Monkeys: {:?}", monkeys);
            }
        }

        monkeys.sort_by(|a, b| b.inspection_amount.cmp(&a.inspection_amount));

        monkeys
            .into_iter()
            .map(|x| x.inspection_amount)
            .take(2)
            .product()
    }
}

pub fn run(input: String) {
    let monkeys = parse(&input);

    let part_1 = Observer::new(20, |x| x / 3).observe(monkeys.clone());

    println!("Day 11");
    println!("  Part 1: {}", part_1);

    let lcm: u64 = monkeys.iter().map(|x| x.test as u64).product();
    let part_2 = Observer::new(10_000, |x| x % lcm).observe(monkeys);
    println!("  Part 2: {}", part_2);
}
