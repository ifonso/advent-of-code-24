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

fn evaluate_direction(m: &Vec<Vec<char>>, pos: &Position, direction: (i32, i32)) -> bool {
    // Assumindo que a primeira posição sempre é X e é válida.

    let height = m.len() as i32 - 1;
    let width = m.first().expect("Empty line").len() as i32 - 1;
    let word = "XMAS";
    let mut current_position = pos.clone();

    for c in word.chars() {
        let c_char = m[current_position.x as usize][current_position.y as usize];
        if c_char != c {
            return false;
        }

        if c == 'S' {
            // Caso a string termine em uma posição que não tenha como ser incrementada, pular
            break;
        }

        if current_position.x + direction.0 > width || current_position.x + direction.0 < 0 {
            return false;
        } else {
            current_position.x += direction.0;
        }

        if current_position.y + direction.1 > height || current_position.y + direction.1 < 0 {
            return false;
        } else {
            current_position.y += direction.1;
        }
    }

    true
}

fn count_word_by_ways(m: &Vec<Vec<char>>, pos: &Position) -> u32 {
    let directions: Vec<(i32, i32)> = Vec::from([
        (-1, -1), // diagonal superior esquerda
        (-1, 0),  // cima
        (-1, 1),  // diagonal superior direita
        (0, -1),  // esquerda
        (0, 1),   // direita
        (1, -1),  // diagonal inferior esquerda
        (1, 0),   // baixo
        (1, 1),   // diagonal inferior direita
    ]);

    let mut valid_words: u32 = 0;

    for d in directions {
        if evaluate_direction(m, pos, d) {
            valid_words += 1;
        }
    }

    println!("pos: {} {} count: {}", pos.x, pos.y, valid_words);

    return valid_words;
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

            if *c == 'X' {
                valid_words += count_word_by_ways(&data, &pos);
            }
        }
    }

    println!("Valud XMAS count: {}", valid_words);
}
