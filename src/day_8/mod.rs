type Matrix = Vec<Vec<u32>>;

fn get_rows(with: &Matrix, at: (usize, usize)) -> (Vec<u32>, Vec<u32>) {
    let column = &with[at.1];

    let before = column[..at.0].to_vec();
    let after = column[at.0 + 1..].to_vec();

    (before, after)
}

fn get_columns(with: &Matrix, at: (usize, usize)) -> (Vec<u32>, Vec<u32>) {
    let before = with[..at.1].iter().map(|x| x[at.0]).collect();

    let after = with[at.1 + 1..].iter().map(|x| x[at.0]).collect();

    (before, after)
}

fn count(with: impl IntoIterator<Item = u32>, at: u32) -> usize {
    let mut steps = 0;

    let iter = with.into_iter();
    let c = iter.collect::<Vec<_>>();
    for number in c {
        steps += 1;

        if number >= at {
            break;
        }
    }

    steps
}

fn count_steps_until_bigger(with: &Vec<u32>, at: u32, rev: bool) -> usize {
    if rev {
        count(with.iter().cloned().rev(), at)
    } else {
        count(with.clone(), at)
    }
}

fn get_scenic_score(rows: (&Vec<u32>, &Vec<u32>), columns: (&Vec<u32>, &Vec<u32>), at: u32) -> u32 {
    let left_row = count_steps_until_bigger(rows.0, at, true);

    let right_row = count_steps_until_bigger(rows.1, at, false);

    let top_column = count_steps_until_bigger(columns.0, at, true);

    let bottom_column = count_steps_until_bigger(columns.1, at, false);

    let score = left_row * right_row * top_column * bottom_column;

    score as u32
}

pub fn run(input: String) {
    let lines = input.lines();

    let mut trees: Vec<Vec<u32>> = Vec::new();

    for (line_index, line) in lines.enumerate() {
        for char in line.chars() {
            if trees.len() <= line_index {
                trees.push(Vec::new());
            }

            let current_tree = &mut trees[line_index];

            if let Some(number) = char.to_digit(10) {
                current_tree.push(number);
            }
        }
    }

    let row_size = trees[0].len();
    let column_size = trees.len();

    let mut visible = (row_size * 2) + (column_size * 2) - 4;

    let mut biggest_senic_score = 0;
    for column_index in 1..column_size - 1 {
        let column = &trees[column_index];

        for row_index in 1..row_size - 1 {
            let current = column[row_index];
            let (before_row, after_row) = get_rows(&trees, (row_index, column_index));

            let less_in_before = before_row.iter().all(|x| x < &current);
            let less_in_after = after_row.iter().all(|x| x < &current);

            let (before_column, after_column) = get_columns(&trees, (row_index, column_index));

            let less_before_column = before_column.iter().all(|x| x < &current);
            let less_after_column = after_column.iter().all(|x| x < &current);

            // Part 1
            if less_in_before || less_in_after || less_before_column || less_after_column {
                visible += 1;
            }

            // Part 2
            let scenic_score = get_scenic_score(
                (&before_row, &after_row),
                (&before_column, &after_column),
                current,
            );
            if scenic_score > biggest_senic_score {
                biggest_senic_score = scenic_score;
            }
        }
    }

    println!("Day 8:");
    println!("  Part 1: {}", visible);
    println!("  Part 2: {}", biggest_senic_score)
}
