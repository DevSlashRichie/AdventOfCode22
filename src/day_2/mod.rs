const DRAW: &i32 = &3;
const WIN : &i32 = &6;

#[derive(Clone, Debug, PartialEq)]
enum Tool {
    Rock,
    Paper,
    Scissors,
}

enum State {
    Draw,
    Win,
    Lose,
}

impl State {
    fn points(&self) -> i32 {
        match self {
            State::Draw => *DRAW,
            State::Win => *WIN,
            State::Lose => 0,
        }
    }
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        match s {
            "X" => State::Lose,
            "Y" => State::Draw,
            "Z" => State::Win,
            _ => panic!("Invalid state"),
        }
    }
}

impl Tool {

    fn opposite(&self) -> Self {
        match self {
            Tool::Rock => Tool::Paper,
            Tool::Paper => Tool::Scissors,
            Tool::Scissors => Tool::Rock,
        }
    }
    
    fn find(&self, state: &State) -> Self {
        match state {
            State::Draw => self.clone(),
            State::Win => self.opposite(),
            State::Lose => self.opposite().opposite(),
        }
    }

    fn beats(&self, other: &Tool) -> bool {
        match (self, other) {
            (Tool::Rock, Tool::Scissors) => true,
            (Tool::Paper, Tool::Rock) => true,
            (Tool::Scissors, Tool::Paper) => true,
            _ => false,
        }
    }

    fn points(&self) -> i32 {
        match self {
            Tool::Rock => 1,
            Tool::Paper => 2,
            Tool::Scissors => 3,
        }
    }
}

impl From<&str> for Tool {

    fn from(s: &str) -> Self {
        match s {
            "X" | "A" => Tool::Rock,
            "Y" | "B" => Tool::Paper,
            "Z" | "C" => Tool::Scissors,
            _ => panic!("Invalid tool"),
        }
    }
}

#[derive(Debug)]
struct Game {
    player_a: Tool,
    secret_b: String,
}

fn get_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let data = line.split_whitespace().collect::<Vec<&str>>();
            Game {
                player_a: data[0].into(),
                secret_b: data[1].to_string(),
            }
        })
        .collect::<Vec<_>>()

}

pub fn run(input: String) {
    let games = get_games(&input);

    let mut a= 0;
    let mut b = 0;
    for game in &games {
        let player_b: Tool = game.secret_b.as_str().into();
        a += game.player_a.points();
        b += player_b.points();

        if game.player_a == player_b {
            a += DRAW;
            b += DRAW;
        } else {
            if game.player_a.beats(&player_b) {
                a += WIN;
            } else {
                b += WIN;
            }
        }
    }

    println!("Day 2:");
    println!("  Part 1:");
    println!("    Player A: {}", a);
    println!("    Player B: {}", b);

    // Part 2
    let mut score = 0;

    for game in games {
        let end_state: State = game.secret_b.as_str().into();
        let tool = game.player_a.find(&end_state);

        score += tool.points();
        score += end_state.points();
    }

    println!("  Part 2:");
    println!("    Score: {}", score);
}
