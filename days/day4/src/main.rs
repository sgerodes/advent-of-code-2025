use std::io::{self, Read};

fn count_accessible_rolls(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                let mut adjacent_rolls = 0;
                
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        
                        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                            if grid[ni as usize][nj as usize] == '@' {
                                adjacent_rolls += 1;
                            }
                        }
                    }
                }
                
                if adjacent_rolls < 4 {
                    count += 1;
                }
            }
        }
    }
    
    count
}

fn solve_part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();
    
    count_accessible_rolls(&grid)
}

fn find_accessible_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut accessible = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                let mut adjacent_rolls = 0;
                
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        
                        if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                            if grid[ni as usize][nj as usize] == '@' {
                                adjacent_rolls += 1;
                            }
                        }
                    }
                }
                
                if adjacent_rolls < 4 {
                    accessible.push((i, j));
                }
            }
        }
    }
    
    accessible
}

fn solve_part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();
    
    let mut total_removed = 0;
    
    loop {
        let accessible = find_accessible_positions(&grid);
        
        if accessible.is_empty() {
            break;
        }
        
        for (i, j) in &accessible {
            grid[*i][*j] = '.';
        }
        
        total_removed += accessible.len() as u32;
    }
    
    total_removed
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

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_example_part1() {
        let input = "\
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            ";
        assert_eq!(solve_part1(input), 13);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(solve_part2(input), 43);
    }
}
