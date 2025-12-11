use std::io::{self, Read};

fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<usize>) {
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let target_str = &line[bracket_start + 1..bracket_end];
    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();
    
    let mut buttons = Vec::new();
    let mut i = bracket_end + 1;
    while i < line.len() {
        if line.chars().nth(i) == Some('(') {
            let start = i + 1;
            let end = line[start..].find(')').unwrap() + start;
            let button_str = &line[start..end];
            let button: Vec<usize> = button_str
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(button);
            i = end + 1;
        } else {
            i += 1;
        }
    }
    
    let brace_start = line.find('{').unwrap();
    let brace_end = line.find('}').unwrap();
    let joltage_str = &line[brace_start + 1..brace_end];
    let joltages: Vec<usize> = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    (target, buttons, joltages)
}

fn find_min_solution_mod2(a: &[Vec<u8>], b: &[u8], free_vars: &[usize], partial: &mut Vec<u8>, best: &mut Option<usize>) {
    if free_vars.is_empty() {
        let mut solution = partial.clone();
        let n = a.len();
        let m = a[0].len();
        
        for r in (0..n).rev() {
            let mut pivot_col = None;
            for c in 0..m {
                if a[r][c] == 1 {
                    pivot_col = Some(c);
                    break;
                }
            }
            
            if let Some(pc) = pivot_col {
                let mut val = b[r];
                for c in (pc + 1)..m {
                    val ^= a[r][c] & solution[c];
                }
                solution[pc] = val;
            } else {
                if b[r] != 0 {
                    return;
                }
            }
        }
        
        let weight: usize = solution.iter().map(|&x| x as usize).sum();
        if best.is_none() || weight < best.unwrap() {
            *best = Some(weight);
        }
        return;
    }
    
    let var = free_vars[0];
    partial[var] = 0;
    find_min_solution_mod2(a, b, &free_vars[1..], partial, best);
    partial[var] = 1;
    find_min_solution_mod2(a, b, &free_vars[1..], partial, best);
    partial[var] = 0;
}

fn solve_system_mod2(a: &mut [Vec<u8>], b: &mut [u8]) -> Option<usize> {
    let n = a.len();
    let m = a[0].len();
    
    let mut row = 0;
    let mut pivot_cols = Vec::new();
    
    for col in 0..m {
        let mut pivot = None;
        for r in row..n {
            if a[r][col] == 1 {
                pivot = Some(r);
                break;
            }
        }
        
        if let Some(p) = pivot {
            if p != row {
                a.swap(row, p);
                let temp = b[row];
                b[row] = b[p];
                b[p] = temp;
            }
            
            for r in (row + 1)..n {
                if a[r][col] == 1 {
                    for c in 0..m {
                        a[r][c] ^= a[row][c];
                    }
                    b[r] ^= b[row];
                }
            }
            pivot_cols.push(col);
            row += 1;
        }
    }
    
    for r in row..n {
        if b[r] != 0 {
            return None;
        }
    }
    
    let mut free_vars = Vec::new();
    for c in 0..m {
        if !pivot_cols.contains(&c) {
            free_vars.push(c);
        }
    }
    
    let mut partial = vec![0; m];
    let mut best = None;
    find_min_solution_mod2(a, b, &free_vars, &mut partial, &mut best);
    best
}

fn solve_machine(target: &[bool], buttons: &[Vec<usize>]) -> Option<usize> {
    let n_lights = target.len();
    let n_buttons = buttons.len();
    
    let mut a = vec![vec![0u8; n_buttons]; n_lights];
    let mut b = vec![0u8; n_lights];
    
    for (light_idx, &should_be_on) in target.iter().enumerate() {
        if should_be_on {
            b[light_idx] = 1;
        }
        
        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&light_idx) {
                a[light_idx][button_idx] = 1;
            }
        }
    }
    
    solve_system_mod2(&mut a, &mut b)
}

fn solve_system_int_simple(joltages: &[usize], buttons: &[Vec<usize>]) -> Option<usize> {
    let mut best = None;
    let mut presses = vec![0; buttons.len()];
    
    fn search(
        buttons: &[Vec<usize>],
        joltages: &[usize],
        presses: &mut Vec<usize>,
        idx: usize,
        current_sum: usize,
        best: &mut Option<usize>,
    ) {
        if best.is_some() && current_sum >= best.unwrap() {
            return;
        }
        
        if idx == buttons.len() {
            let mut counters = vec![0; joltages.len()];
            for (button_idx, &press_count) in presses.iter().enumerate() {
                for &counter_idx in &buttons[button_idx] {
                    counters[counter_idx] += press_count;
                }
            }
            
            if counters == joltages {
                if best.is_none() || current_sum < best.unwrap() {
                    *best = Some(current_sum);
                }
            }
            return;
        }
        
        let mut remaining = joltages.to_vec();
        for (button_idx, &press_count) in presses.iter().enumerate() {
            if button_idx < idx {
                for &counter_idx in &buttons[button_idx] {
                    remaining[counter_idx] = remaining[counter_idx].saturating_sub(press_count);
                }
            }
        }
        
        let mut max_needed = 0;
        for &counter_idx in &buttons[idx] {
            max_needed = max_needed.max(remaining[counter_idx]);
        }
        
        let upper = max_needed.min(best.unwrap_or(usize::MAX).saturating_sub(current_sum));
        for val in 0..=upper {
            presses[idx] = val;
            search(buttons, joltages, presses, idx + 1, current_sum + val, best);
            if best.is_some() && current_sum + val >= best.unwrap() {
                break;
            }
        }
        presses[idx] = 0;
    }
    
    search(buttons, joltages, &mut presses, 0, 0, &mut best);
    best
}

fn solve_machine_joltage(joltages: &[usize], buttons: &[Vec<usize>]) -> Option<usize> {
    solve_system_int_simple(joltages, buttons)
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (target, buttons, _) = parse_machine(line);
            solve_machine(&target, &buttons).unwrap_or(0)
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (_, buttons, joltages) = parse_machine(line);
            solve_machine_joltage(&joltages, &buttons).unwrap_or(0)
        })
        .sum()
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
