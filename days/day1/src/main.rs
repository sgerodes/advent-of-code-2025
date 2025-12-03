use std::io::{self, Read};

fn count_zeros_during_rotation(start: i32, distance: i32, is_left: bool) -> u32 {
    const MOD: i32 = 100;
    let mut count = 0u32;
    
    if is_left {
        for step in 1..=distance {
            let pos = (start - step).rem_euclid(MOD);
            if pos == 0 {
                count += 1;
            }
        }
    } else {
        for step in 1..=distance {
            let pos = (start + step).rem_euclid(MOD);
            if pos == 0 {
                count += 1;
            }
        }
    }
    
    count
}

fn solve_part1(input: &str) -> u32 {
    const MOD: i32 = 100;
    let mut position: i32 = 50;
    let mut zeros: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir_char, rest) = line.split_at(1);
        let distance: i32 = match rest.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        match dir_char {
            "L" => {
                position = (position - distance).rem_euclid(MOD);
            }
            "R" => {
                position = (position + distance).rem_euclid(MOD);
            }
            _ => continue,
        }

        if position == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn solve_part2(input: &str) -> u32 {
    const MOD: i32 = 100;
    let mut position: i32 = 50;
    let mut zeros: u32 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (dir_char, rest) = line.split_at(1);
        let distance: i32 = match rest.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let is_left = dir_char == "L";
        
        zeros += count_zeros_during_rotation(position, distance, is_left);

        match dir_char {
            "L" => {
                position = (position - distance).rem_euclid(MOD);
            }
            "R" => {
                position = (position + distance).rem_euclid(MOD);
            }
            _ => continue,
        }
    }

    zeros
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
    use super::{solve_part1, solve_part2, count_zeros_during_rotation};

    #[test]
    fn test_example_part1() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(solve_part2(input), 6);
    }

    #[test]
    fn test_zeros_during_rotation() {
        assert_eq!(count_zeros_during_rotation(50, 68, true), 1);
        assert_eq!(count_zeros_during_rotation(55, 60, false), 1);
        assert_eq!(count_zeros_during_rotation(14, 82, true), 1);
        assert_eq!(count_zeros_during_rotation(50, 1000, false), 10);
    }
}
