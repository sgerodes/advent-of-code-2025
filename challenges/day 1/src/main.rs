use std::io::{self, Read};

fn solve(input: &str) -> u32 {
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

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let answer = solve(&input);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_from_description() {
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
        assert_eq!(solve(input), 3);
    }
}
