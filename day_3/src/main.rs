// temos tres estados no processamento de uma entrada,
// lê-se o comando que deve ser mul
// a partir do comando começa a captura da entrada com (
// a partir desse dígito checar pelo primeiro dígito até encontrar o separador ,
// após encontrar o separador, esperar que 
//
// se encontrar m, prosseguir até formar mul, se em algum índice encontrar uma operação inválida,
// começar novamente o processo a partir da posição inválida.

fn check_substring(sub: &str) -> (usize, i32) {
    if sub.len() < 5 {
        return (sub.len() - 1, 0);
    }
    return (1, 2);
}

fn main() {
    let filepath = "input.md";
    let data = std::fs::read_to_string(filepath)
        .expect("Could not read input file");

    let mut current_index: usize = 0;
    let mut cumulator: i32 = 0;

    while current_index < data.len() {
        if let Some(c) = data.chars().nth(current_index) {
            if c != 'm'{
                current_index += 1;
                continue;
            }

            if current_index + 2 > data.len() {
                break;
            }

            let next_chars: String = data.chars()
                .skip(current_index + 1)
                .take(2)
                .collect();

            if next_chars == "ul" {
                let substring = &data[current_index + 3..];
                let (index, sum) = check_substring(substring);
                
                cumulator += sum;
                current_index += 3 + index;
                continue;
            }

            current_index += 1;
        }
    }
}
