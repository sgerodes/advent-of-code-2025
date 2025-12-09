use std::io::{self, Read};

fn find_problems(lines: &[&str]) -> Vec<(usize, usize)> {
    if lines.is_empty() {
        return Vec::new();
    }

    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let num_cols = max_width;

    let mut separator_cols = vec![false; num_cols];
    for col in 0..num_cols {
        separator_cols[col] = lines.iter().all(|line| {
            col >= line.len() || line.chars().nth(col).unwrap_or(' ') == ' '
        });
    }

    let mut problems = Vec::new();
    let mut current_problem_start = None;

    for col in 0..num_cols {
        if separator_cols[col] {
            if let Some(start) = current_problem_start {
                problems.push((start, col));
                current_problem_start = None;
            }
        } else {
            if current_problem_start.is_none() {
                current_problem_start = Some(col);
            }
        }
    }
    if let Some(start) = current_problem_start {
        problems.push((start, num_cols));
    }

    problems
}

fn solve_part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let num_rows = lines.len();
    let problems = find_problems(&lines);

    let mut grand_total = 0i64;
    for (start_col, end_col) in problems {
        let mut numbers = Vec::new();
        let mut operation = None;

        for row in 0..num_rows {
            let line = lines[row];
            let mut num_str = String::new();
            
            for col in start_col..end_col.min(line.len()) {
                if let Some(ch) = line.chars().nth(col) {
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                    }
                }
            }

            if row == num_rows - 1 {
                for col in start_col..end_col.min(line.len()) {
                    if let Some(ch) = line.chars().nth(col) {
                        if ch == '*' || ch == '+' {
                            operation = Some(ch);
                            break;
                        }
                    }
                }
            } else {
                if !num_str.is_empty() {
                    if let Ok(num) = num_str.parse::<i64>() {
                        numbers.push(num);
                    }
                }
            }
        }

        if let Some(op) = operation {
            let result = if op == '*' {
                numbers.iter().product::<i64>()
            } else {
                numbers.iter().sum::<i64>()
            };
            grand_total += result;
        }
    }

    grand_total
}

fn solve_part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let num_rows = lines.len();
    let problems = find_problems(&lines);

    let mut grand_total = 0i64;
    for (start_col, end_col) in problems {
        let mut numbers = Vec::new();
        let mut operation = None;

        for col in start_col..end_col.min(lines[num_rows - 1].len()) {
            if let Some(ch) = lines[num_rows - 1].chars().nth(col) {
                if ch == '*' || ch == '+' {
                    operation = Some(ch);
                    break;
                }
            }
        }

        for col in (start_col..end_col).rev() {
            let mut num_str = String::new();
            let mut has_digit = false;
            
            for row in 0..(num_rows - 1) {
                if col < lines[row].len() {
                    if let Some(ch) = lines[row].chars().nth(col) {
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                            has_digit = true;
                        }
                    }
                }
            }
            
            if has_digit && !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        if let Some(op) = operation {
            let result = if op == '*' {
                numbers.iter().product::<i64>()
            } else {
                numbers.iter().sum::<i64>()
            };
            grand_total += result;
        }
    }

    grand_total
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || (args[1] != "part1" && args[1] != "part2") {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        eprintln!("Example: {} part1 < input.txt", args[0]);
        std::process::exit(1);
    }
    
    let part = &args[1];

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let answer = if part == "part1" {
        solve_part1(&input)
    } else {
        solve_part2(&input)
    };
    
    println!("{answer}");
}
