use std::fs;

fn is_valid(report: &[u32]) -> bool {
    let is_increasing: bool = report[0] < report[1];
    let mut is_safe: bool = true;
    let mut last: u32 = 0;

    for (i, &val) in report.iter().enumerate() {
        if i == 0 {
            last = val;
            continue;
        }

        if is_increasing {
            if last >= val || (val - last) > 3 {
                is_safe = false;
                break;
            }
        } else {
            if last <= val || (last - val) > 3 {
                is_safe = false;
                break;
            }
        }

        last = val;
    }

    return is_safe;
}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Could not read input.txt");

    let mut valid_reports_count: u32 = 0;

    for line in data.lines() {
        let report = line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_valid(&report) {
            valid_reports_count += 1;
            continue;
        }

        // Parte 2

        let mut was_valid_removing: bool = false;

        for i in 0..report.len() {
            let report_copy = report.iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, v)| v.clone())
                .collect::<Vec<u32>>();

            if is_valid(&report_copy) {
                was_valid_removing = true;
                break;
            }
        }

        if was_valid_removing {
            valid_reports_count += 1;
        }
    }

    println!("Valid reports: {}", valid_reports_count);
}
