use std::io::{self, Read};

fn is_invalid_id_part1(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    
    if len % 2 != 0 {
        return false;
    }
    
    let half = len / 2;
    &s[..half] == &s[half..]
}

fn is_invalid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    
    if len < 2 {
        return false;
    }
    
    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;
        
        if repetitions < 2 {
            continue;
        }
        
        let reconstructed: String = pattern.repeat(repetitions);
        if reconstructed == s {
            return true;
        }
    }
    
    false
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || (args[1] != "part1" && args[1] != "part2") {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        eprintln!("Example: {} part1 < input.txt", args[0]);
        std::process::exit(1);
    }
    
    let part = &args[1];
    let is_invalid = if part == "part1" {
        is_invalid_id_part1
    } else {
        is_invalid_id_part2
    };

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let input = input.trim();
    let mut sum = 0u64;

    for range_str in input.split(',') {
        let range_str = range_str.trim();
        if let Some(dash_pos) = range_str.find('-') {
            let start: u64 = range_str[..dash_pos].parse().expect("invalid start");
            let end: u64 = range_str[dash_pos + 1..].parse().expect("invalid end");
            
            for id in start..=end {
                if is_invalid(id) {
                    sum += id;
                }
            }
        }
    }

    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::{is_invalid_id_part1, is_invalid_id_part2};

    #[test]
    fn test_invalid_ids_part1() {
        assert!(is_invalid_id_part1(11));
        assert!(is_invalid_id_part1(22));
        assert!(is_invalid_id_part1(55));
        assert!(is_invalid_id_part1(99));
        assert!(is_invalid_id_part1(6464));
        assert!(is_invalid_id_part1(1010));
        assert!(is_invalid_id_part1(123123));
        assert!(is_invalid_id_part1(222222));
        assert!(is_invalid_id_part1(446446));
        assert!(is_invalid_id_part1(38593859));
        assert!(is_invalid_id_part1(1188511885));
        
        assert!(!is_invalid_id_part1(111));
        assert!(!is_invalid_id_part1(999));
        assert!(!is_invalid_id_part1(565656));
        assert!(!is_invalid_id_part1(824824824));
        assert!(!is_invalid_id_part1(2121212121));
    }

    #[test]
    fn test_invalid_ids_part2() {
        assert!(is_invalid_id_part2(11));
        assert!(is_invalid_id_part2(22));
        assert!(is_invalid_id_part2(55));
        assert!(is_invalid_id_part2(99));
        assert!(is_invalid_id_part2(111));
        assert!(is_invalid_id_part2(999));
        assert!(is_invalid_id_part2(6464));
        assert!(is_invalid_id_part2(1010));
        assert!(is_invalid_id_part2(123123));
        assert!(is_invalid_id_part2(12341234));
        assert!(is_invalid_id_part2(123123123));
        assert!(is_invalid_id_part2(1212121212));
        assert!(is_invalid_id_part2(1111111));
        assert!(is_invalid_id_part2(222222));
        assert!(is_invalid_id_part2(446446));
        assert!(is_invalid_id_part2(38593859));
        assert!(is_invalid_id_part2(1188511885));
        assert!(is_invalid_id_part2(565656));
        assert!(is_invalid_id_part2(824824824));
        assert!(is_invalid_id_part2(2121212121));
    }

    #[test]
    fn test_valid_ids() {
        assert!(!is_invalid_id_part1(10));
        assert!(!is_invalid_id_part1(100));
        assert!(!is_invalid_id_part1(101));
        assert!(!is_invalid_id_part1(123));
        assert!(!is_invalid_id_part1(1234));
        
        assert!(!is_invalid_id_part2(10));
        assert!(!is_invalid_id_part2(100));
        assert!(!is_invalid_id_part2(101));
        assert!(!is_invalid_id_part2(123));
        assert!(!is_invalid_id_part2(1234));
        assert!(!is_invalid_id_part2(12345));
    }
}
