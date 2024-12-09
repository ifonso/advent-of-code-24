use std::char;

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

fn evaluete_moviment(m: &[Vec<char>], pos: &Position, start_point: (i32, i32), mov: (i32, i32)) -> bool {
    let height = m.len() as i32 - 1;
    let width = m.first().expect("Empty line").len() as i32 - 1;
    let word = "MAS";

    if pos.x + start_point.0 > width || pos.x + start_point.0 < 0 {
        return false;
    }

    if pos.y + start_point.1 > height || pos.y + start_point.1 < 0 {
        return false;
    }

    let mut current_position = Position::new(pos.x + start_point.0, pos.y + start_point.1);

    for c in word.chars() {
        let c_char = m[current_position.x as usize][current_position.y as usize];
        if c != c_char {
            return false;
        }

        if c == 'S' {
            break;
        }

        if current_position.x + mov.0 > width || current_position.x + mov.0 < 0 {
            return false;
        }

        if current_position.y + mov.1 > height || current_position.y + mov.1 < 0 {
            return false;
        }

        current_position.x += mov.0;
        current_position.y += mov.1;
    }

    true
}

fn evaluate_word(m: &[Vec<char>], pos: &Position) -> bool {
    let starting_directions: Vec<(i32, i32)> = Vec::from([
        (-1, -1), (1, -1), (1, 1), (-1, 1)
    ]);

    let moving_directions: Vec<(i32, i32)> = Vec::from([
        (1, 1), (-1, 1), (-1, -1), (1, -1)
    ]);
    
    let mut match_counter: u32 = 0;

    for (i, d) in starting_directions.iter().enumerate() {
        if evaluete_moviment(m, pos, *d, moving_directions[i]) {
            match_counter += 1;
        }
    }

    match_counter == 2
}

fn main() {
    let filepath = "./input.md";
    let data = std::fs::read_to_string(filepath)
        .expect("Couldn't read input file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut valid_words: u32 = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let pos = Position::new(i as i32, j as i32);

            if *c == 'A' && evaluate_word(&data, &pos) {
                valid_words += 1;
            }
        }
    }

    println!("Valud XMAS count: {}", valid_words);
}
