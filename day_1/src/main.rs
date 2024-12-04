use std::collections::HashMap;
use std::fs;
use std::io;

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let mut nums = line
        .split_whitespace()
        .filter_map(|v| v.parse::<i32>().ok());

    match (nums.next_back(), nums.next_back()) {
        (Some(right), Some(left)) => Some((right, left)),
        _ => None,
    }
}

// Cria um dicionário com os valores da esquerda como chave e a quantidade de repetições
// a direita como valor.
fn calculate_similarity(left_values: &[i32], right_values: &[i32]) -> HashMap<i32, i32> {
    let mut similarity_score = HashMap::new();
    // Coerção de desreferência
    for &v in left_values {
        similarity_score
            // retorna o correspondente a V ou um manipulador para inserir V
            .entry(v)
            .or_insert_with(|| right_values.iter().filter(|&&x| x == v).count() as i32);
    }

    similarity_score
}

fn main() -> io::Result<()> {
    let filepath: &str = "input.md";

    let data = fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
        .lines()
        .filter_map(parse_line)
        .collect::<Vec<(i32, i32)>>();

    let mut left_values: Vec<i32> = Vec::new();
    let mut right_values: Vec<i32> = Vec::new();

    for (r, l) in &data {
        right_values.push(*r);
        left_values.push(*l);
    }

    left_values.sort_unstable();
    right_values.sort_unstable();

    let coordinate_similarity_score = calculate_similarity(&left_values, &right_values);
    let similarity_total: i32 = coordinate_similarity_score
        .into_iter()
        .map(|(k, v)| k * v)
        .sum();

    println!("Program result: {}", similarity_total);
    Ok(())
}
