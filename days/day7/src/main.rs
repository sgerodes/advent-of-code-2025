use std::io::{self, Read};
use std::collections::HashSet;

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    if grid.is_empty() {
        return 0;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut start_pos = None;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_pos = Some((r, c));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }
    
    let (start_r, start_c) = start_pos.expect("No S found");
    
    let mut active_beams = HashSet::new();
    active_beams.insert((start_r, start_c));
    
    let mut split_count = 0;
    
    for row in (start_r + 1)..rows {
        let mut next_beams = HashSet::new();
        
        for &(_r, c) in &active_beams {
            if grid[row][c] == '^' {
                split_count += 1;
                
                if c > 0 {
                    next_beams.insert((row, c - 1));
                }
                
                if c + 1 < cols {
                    next_beams.insert((row, c + 1));
                }
            } else {
                next_beams.insert((row, c));
            }
        }
        
        active_beams = next_beams;
    }
    
    split_count
}

fn solve_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    if grid.is_empty() {
        return 0;
    }
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut start_pos = None;
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_pos = Some((r, c));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }
    
    let (start_r, start_c) = start_pos.expect("No S found");
    
    let mut position_counts: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
    position_counts.insert((start_r, start_c), 1);
    
    for row in (start_r + 1)..rows {
        let mut next_counts: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        
        for (&(_r, c), &count) in &position_counts {
            let cell = grid[row][c];
            if cell == '^' {
                if c > 0 {
                    *next_counts.entry((row, c - 1)).or_insert(0) += count;
                }
                if c + 1 < cols {
                    *next_counts.entry((row, c + 1)).or_insert(0) += count;
                }
            } else if cell == '.' || cell == 'S' {
                *next_counts.entry((row, c)).or_insert(0) += count;
            }
        }
        
        position_counts = next_counts;
    }
    
    position_counts.values().sum()
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
