enum IterationState {
    CollectFirst,
    CollectSecond,
}

fn check_substring(sub: &str) -> (usize, i32) {
    if sub.len() < 5 {
        return (sub.len() - 1, 0);
    }

    if sub.chars().nth(0).unwrap() != '(' {
        return (1, 0);
    }

    let mut state: IterationState = IterationState::CollectFirst;
    let mut fst_n: String = String::new();
    let mut sec_n: String = String::new();
    let mut idx: usize = 0;

    for (i, ch) in sub.char_indices() {
        if i == 0 {
            continue;
        }
        
        idx = i;

        if let IterationState::CollectFirst = state {
            if ch.is_numeric() {
                // Number found
                fst_n.push(ch);
                continue;
            } else if ch == ',' {
                // End of first value
                if fst_n.is_empty() {
                    return (i, 0);
                }
                state = IterationState::CollectSecond;
                continue;
            } else {
                // Invalid value found
                return (i, 0);
            }
        } else {
            if ch.is_numeric() {
                // Number found
                sec_n.push(ch);
                continue;
            } else if ch == ')' {
                // End of second value
                if sec_n.is_empty() {
                    return (i, 0);
                }
                // End of second value
                return (
                    i,
                    fst_n.parse::<i32>().unwrap() * sec_n.parse::<i32>().unwrap(),
                );
            } else {
                return (i, 0);
            }
        }
    }

    return (idx, 0);
}

// use regex::Regex;
//
// fn find_with_regex(input: &str) -> Vec<String> {
//     let pattern = r"mul\(\d+,\d+\)";
//     let re = Regex::new(pattern).expect("Failed to compile regex");
//
//     re.find_iter(input)
//         .map(|m| m.as_str().to_string())
//         .collect()
// }

fn main() {
    let filepath = "input.md";
    let data = std::fs::read_to_string(filepath).expect("Could not read input file");

    let mut current_index: usize = 0;
    let mut cumulator: i32 = 0;

    while current_index < data.len() {
        if let Some(c) = data.chars().nth(current_index) {
            // Checks first match
            if c != 'm' {
                current_index += 1;
                continue;
            }

            // If first match ok, check size for next matches
            if current_index + 3 > data.len() - 1 {
                break;
            }

            let next_chars = data
                .chars()
                .skip(current_index + 1)
                .take(2)
                .collect::<String>();

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

    println!("{}", cumulator);
}
