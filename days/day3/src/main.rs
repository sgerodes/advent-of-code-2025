use std::io::{self, Read};

fn max_joltage(bank: &str) -> u64 {
    let chars: Vec<char> = bank.chars().collect();
    let len = chars.len();
    
    if len < 2 {
        panic!("Bank must have at least 2 digits");
    }
    
    let mut max_joltage = 0u64;
    
    for i in 0..len - 1 {
        let d1 = chars[i].to_digit(10).expect("Expected digit");
        for j in (i + 1)..len {
            let d2 = chars[j].to_digit(10).expect("Expected digit");
            let joltage = (d1 * 10 + d2) as u64;
            if joltage > max_joltage {
                max_joltage = joltage;
            }
        }
    }
    
    max_joltage
}


fn solve_part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        if !line.is_empty() {
            sum += max_joltage(line);
        }
    }
    sum
}

fn max_joltage_part2(bank: &str) -> u64 {
    let chars: Vec<char> = bank.chars().collect();
    let len = chars.len();
    const TARGET_DIGITS: usize = 12;
    
    if len < TARGET_DIGITS {
        panic!("Bank must have at least {} digits", TARGET_DIGITS);
    }
    
    let mut selected_digits = Vec::new();
    let mut start_idx = 0;
    
    for pos in 0..TARGET_DIGITS {
        let remaining_positions = TARGET_DIGITS - pos - 1;
        let end_idx = len - remaining_positions;
        
        let mut max_digit = 0u32;
        let mut max_idx = start_idx;
        
        for i in start_idx..end_idx {
            let digit = chars[i].to_digit(10).expect("Expected digit");
            if digit > max_digit {
                max_digit = digit;
                max_idx = i;
            }
            if max_digit == 9 {
                break;
            }
        }
        
        selected_digits.push(max_digit);
        start_idx = max_idx + 1;
    }
    
    let mut result = 0u64;
    for digit in selected_digits {
        result = result * 10 + digit as u64;
    }
    
    result
}

fn solve_part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        if !line.is_empty() {
            sum += max_joltage_part2(line);
        }
    }
    sum
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
    use super::{max_joltage, solve_part1, solve_part2};

    #[test]
    fn test_max_joltage_examples() {
        assert_eq!(max_joltage("987654321111111"), 98);
        assert_eq!(max_joltage("811111111111119"), 89);
        assert_eq!(max_joltage("234234234234278"), 78);
        assert_eq!(max_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_example_part1() {
        let input = "\
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            ";
        assert_eq!(solve_part1(input), 98 + 89 + 78 + 92);
    }

    #[test]
    fn test_max_joltage_part2_examples() {
        use super::max_joltage_part2;
        assert_eq!(max_joltage_part2("987654321111111"), 987654321111);
        assert_eq!(max_joltage_part2("811111111111119"), 811111111119);
        assert_eq!(max_joltage_part2("234234234234278"), 434234234278);
        assert_eq!(max_joltage_part2("818181911112111"), 888911112111);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            ";
        assert_eq!(solve_part2(input), 987654321111 + 811111111119 + 434234234278 + 888911112111);
    }
}
