use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    vec,
};

#[derive(Debug)]
struct Land<T> {
    matrix: Vec<Vec<Option<T>>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl<T: Clone> Land<T> {
    fn new() -> Self {
        Self {
            matrix: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }
}
impl<T: ToString> ToString for Land<T> {
    fn to_string(&self) -> String {
        self.matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.as_ref().map(|x| x.to_string()).unwrap_or(" ".to_string()))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl FromIterator<Vec<Mark>> for Land<Mark> {
    fn from_iter<T: IntoIterator<Item = Vec<Mark>>>(iter: T) -> Self {
        iter.into_iter().fold(Land::new(), |mut acc, n| {
            let mut position = n.iter().filter(|x| matches!(x, Mark::Position(_)));

            while let Some(Mark::Position(position)) = position.next() {
                let coords = (
                    acc.matrix.len(),
                    n.iter().position(|x| x == position).unwrap(),
                );
                match position {
                    Position::Start => acc.start = coords,
                    Position::End => acc.end = coords,
                }
            }

            acc.insert_row(n)
        })
    }
}
impl<A> Land<A> {
    fn insert_row(mut self, line: Vec<A>) -> Self {
        let l = line.into_iter().map(|x| Some(x)).collect();
        self.matrix.push(l);

        self
    }

    fn neighbors_of(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if row > 0 {
            neighbors.push((row - 1, col));
        }

        if row < self.matrix.len() - 1 {
            neighbors.push((row + 1, col));
        }

        if col > 0 {
            neighbors.push((row, col - 1));
        }

        if col < self.matrix[0].len() - 1 {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

    fn get(&self, loc: (usize, usize)) -> Option<&A> {
        self.matrix[loc.0][loc.1].as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Position {
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mark {
    Position(Position),
    Elevation(char),
}

impl Mark {
    fn cost(&self) -> u8 {
        match self {
            Mark::Elevation(x) => *x as u8 - b'a',
            Mark::Position(pos) => match pos {
                Position::Start => 0,
                Position::End => b'z' - b'a',
            },
        }
    }
}

impl PartialEq<Position> for Mark {
    fn eq(&self, other: &Position) -> bool {
        match (self, other) {
            (Self::Position(x), y) => x == y,
            _ => false,
        }
    }
}

impl ToString for Mark {
    fn to_string(&self) -> String {
        match self {
            Mark::Position(Position::Start) => "S".to_string(),
            Mark::Position(Position::End) => "E".to_string(),
            Mark::Elevation(x) => x.to_string(),
        }
    }
}

fn parse_line_char(char: char) -> Mark {
    match char {
        'S' => Mark::Position(Position::Start),
        'E' => Mark::Position(Position::End),
        x => Mark::Elevation(x),
    }
}

fn parse_line(line: &str) -> Vec<Mark> {
    line.chars().map(parse_line_char).collect()
}

fn parse(line: &str) -> Land<Mark> {
    line.lines().map(|line| parse_line(line)).collect()
}

pub fn run(input: String) {
    let land = parse(&input);

    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visist = VecDeque::new();

    // This algorithm it's bfs

    // part 1
    distances.insert(land.start, 0);
    to_visist.push_back((land.start, 0));

    // part 2
    for x in 0..land.matrix.len() {
        for y in 0..land.matrix[0].len() {
            if land.get((x, y)).unwrap().cost() == 0 {
                distances.insert((x, y), 0);
                to_visist.push_back(((x, y), 0));
            }
        }
    }

    while let Some((u, dc)) = to_visist.pop_front() {
        if !visited.insert(u) {
            continue;
        }

        if u == land.end {
            println!("Found it! {}", dc);
            break;
        }

        for n in land.neighbors_of(u.0, u.1) {
            let nc = land.get(n).unwrap().cost() as u32;
            let uc = land.get(u).unwrap().cost() as u32;

            let new_distance = dc + 1;
            let is_shorter = distances.get(&n).map_or(true, |p| new_distance < *p);

            if is_shorter && nc <= uc + 1 {
                distances.insert(n, new_distance);
                to_visist.push_back((n, new_distance));
            }
        }
    }
}
