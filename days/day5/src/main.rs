use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        
        if let Some(dash_pos) = line.find('-') {
            let start: u64 = line[..dash_pos].parse().expect("Invalid range start");
            let end: u64 = line[dash_pos + 1..].parse().expect("Invalid range end");
            ranges.push(Range { start, end });
        }
    }
    
    ranges
}

fn parse_ingredient_ids(input: &str) -> Vec<u64> {
    let mut in_ids_section = false;
    let mut ids = Vec::new();
    
    for line in input.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            in_ids_section = true;
            continue;
        }
        
        if in_ids_section {
            if let Ok(id) = line.parse::<u64>() {
                ids.push(id);
            }
        }
    }
    
    ids
}

fn is_fresh(id: u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|range| range.contains(id))
}

fn solve_part1(input: &str) -> u32 {
    let ranges = parse_ranges(input);
    let ingredient_ids = parse_ingredient_ids(input);
    
    ingredient_ids.iter()
        .filter(|&&id| is_fresh(id, &ranges))
        .count() as u32
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return ranges;
    }
    
    ranges.sort_by_key(|r| r.start);
    
    let mut merged = Vec::new();
    let mut current = ranges[0];
    
    for range in ranges.into_iter().skip(1) {
        if range.start <= current.end + 1 {
            current.end = current.end.max(range.end);
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);
    
    merged
}

fn solve_part2(input: &str) -> u64 {
    let ranges = parse_ranges(input);
    let merged = merge_ranges(ranges);
    
    merged.iter()
        .map(|range| (range.end - range.start + 1))
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
        solve_part1(&input) as u64
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
            ";
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
            ";
        assert_eq!(solve_part2(input), 14);
    }
}
