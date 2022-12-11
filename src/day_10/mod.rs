use std::collections::BTreeMap;

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct CycleTracker {
    execute_at: usize,
    current_cycle: usize,
    adx: Option<i32>,
}

impl From<Command> for CycleTracker {
    fn from(value: Command) -> Self {
        match value {
            Command::Noop => Self {
                execute_at: 1,
                current_cycle: 0,
                adx: None,
            },
            Command::Addx(adx) => Self {
                execute_at: 2,
                current_cycle: 0,
                adx: Some(adx),
            },
        }
    }
}

impl CycleTracker {
    fn executable(&self) -> bool {
        self.current_cycle == self.execute_at
    }

    fn cycle(&mut self) {
        self.current_cycle += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrtPixel {
    Empty,
    Lit,
    Dark,
}

impl ToString for CrtPixel {
    fn to_string(&self) -> String {
        match self {
            CrtPixel::Empty => " ".to_string(),
            CrtPixel::Lit => "█".to_string(),
            CrtPixel::Dark => "░".to_string(),
        }
    }
}

struct Crt {
    screen: [[CrtPixel; 40]; 6],
    sprite: [CrtPixel; 40],
    current_pixel: usize,
}

fn pixels_to_string(pixels: &[CrtPixel]) -> String {
    pixels.iter().map(|p| p.to_string()).collect()
}

impl Crt {
    fn new() -> Self {
        let mut crt = Self {
            screen: [[CrtPixel::Empty; 40]; 6],
            sprite: [CrtPixel::Dark; 40],
            current_pixel: 0,
        };

        crt.set_sprite(1);

        crt
    }

    fn set_sprite(&mut self, position: usize) {
        if position > 40 || position == 0 {
            panic!("Invalid position: {}", position);
        }

        let position = position - 1;

        let mut new_sprite = [CrtPixel::Dark; 40];

        for i in 0..3 {
            let pos = position + i;
            if pos < 40 {
                new_sprite[pos] = CrtPixel::Lit;
            }
        }

        self.sprite = new_sprite;
    }

    fn draw_next_pixel(&mut self) {
        // Maybe this should be done we a track for the column.
        // Magic maths for now I guess

        let screen_index = (self.current_pixel as f64 / 40_f64).floor() as usize;
        let pixel_index = self.current_pixel % 40;

        let pixel = self.sprite[pixel_index];
        self.screen[screen_index][pixel_index] = pixel;

        self.current_pixel += 1;
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.screen.iter() {
            writeln!(f, "{}", pixels_to_string(row))?;
        }

        Ok(())
    }
}

fn parse_line(line: &str) -> Command {
    let mut split = line.split_whitespace();

    let command = split.next().unwrap();

    let cmd = match command {
        "noop" => Some(Command::Noop),
        "addx" => split.next().map(|n| Command::Addx(n.parse().unwrap())),
        _ => None,
    };

    if let Some(cmd) = cmd {
        cmd
    } else {
        panic!("Invalid command: {}", command);
    }
}

pub fn run(input: String) {
    let commands = input.lines().map(parse_line).collect::<Vec<_>>();

    let mut register = 1;
    let mut cycle_tracker: Option<CycleTracker> = None;

    let mut commands = commands.into_iter();

    // We use BTreemap to keep the order of the keys - I want the order just because yes
    let mut strength_tracker =
        BTreeMap::from([(20, 0), (60, 0), (100, 0), (140, 0), (180, 0), (220, 0)]);

    let mut cycle_counter = 0;

    let mut crt = Crt::new();

    loop {
        // First we try to start a new cycle
        if cycle_tracker.is_none() {
            if let Some(next_command) = commands.next() {
                cycle_tracker.replace(next_command.into());
            } else {
                // If not next comand is found, we finish the program
                break;
            }
        }

        // If a new cycle is started, we increment the cycle counter
        cycle_counter += 1;

        // During the cycle
        if let Some(entry) = strength_tracker.get_mut(&cycle_counter) {
            *entry = cycle_counter * register;
        }

        crt.draw_next_pixel();

        // Ending the cycle.
        if let Some(tracker) = cycle_tracker.as_mut() {
            tracker.cycle();
            if tracker.executable() {
                if let Some(adx) = tracker.adx {
                    register += adx;
                }

                crt.set_sprite(register.try_into().unwrap_or(1));
                cycle_tracker = None;
            }
        } // End of the cycle
    }

    println!("Day 10:");
    let part_1: i32 = strength_tracker.values().sum();
    println!("  Part 1: {}", part_1);
    println!("  Part 2:\n{}", crt);
}
