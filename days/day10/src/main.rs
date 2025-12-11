use std::io::{self, Read};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Fraction {
    num: i128,
    den: i128,
}

impl Fraction {
    fn new(num: i128, den: i128) -> Self {
        let mut n = num;
        let mut d = den;
        if d < 0 {
            n = -n;
            d = -d;
        }
        let g = gcd(n, d);
        Fraction {
            num: n / g,
            den: d / g,
        }
    }

    fn from_i64(v: i64) -> Self {
        Fraction { num: v as i128, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_integer(&self) -> bool {
        self.num % self.den == 0
    }

    fn to_i64(&self) -> Option<i64> {
        if self.is_integer() {
            Some((self.num / self.den) as i64)
        } else {
            None
        }
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

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

fn gaussian_rref(a: &[Vec<i64>], b: &[i64]) -> Option<(Vec<Vec<Fraction>>, Vec<usize>)> {
    let n = a.len();
    let m = a[0].len();
    let mut mat = vec![vec![Fraction::from_i64(0); m + 1]; n];
    for r in 0..n {
        for c in 0..m {
            mat[r][c] = Fraction::from_i64(a[r][c]);
        }
        mat[r][m] = Fraction::from_i64(b[r]);
    }
    let mut row = 0;
    let mut pivot_cols = Vec::new();
    for col in 0..m {
        let mut pivot = None;
        for r in row..n {
            if !mat[r][col].is_zero() {
                pivot = Some(r);
                break;
            }
        }
        if let Some(p) = pivot {
            mat.swap(row, p);
            let pivot_val = mat[row][col].clone();
            for c in col..=m {
                mat[row][c] = mat[row][c].clone() / pivot_val.clone();
            }
            for r in 0..n {
                if r != row && !mat[r][col].is_zero() {
                    let factor = mat[r][col].clone();
                    for c in col..=m {
                        mat[r][c] = mat[r][c].clone() - factor.clone() * mat[row][c].clone();
                    }
                }
            }
            pivot_cols.push(col);
            row += 1;
        }
    }
    for r in 0..n {
        let zero_row = (0..m).all(|c| mat[r][c].is_zero());
        if zero_row && !mat[r][m].is_zero() {
            return None;
        }
    }
    Some((mat, pivot_cols))
}

fn solve_machine_joltage(joltages: &[usize], buttons: &[Vec<usize>]) -> Option<usize> {
    let n_counters = joltages.len();
    let n_buttons = buttons.len();
    let mut a = vec![vec![0i64; n_buttons]; n_counters];
    for (button_idx, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            a[counter_idx][button_idx] = 1;
        }
    }
    let b: Vec<i64> = joltages.iter().map(|&v| v as i64).collect();
    let (mat, pivot_cols) = gaussian_rref(&a, &b)?;
    let mut is_pivot = vec![false; n_buttons];
    for &c in &pivot_cols {
        is_pivot[c] = true;
    }
    let free_cols: Vec<usize> = (0..n_buttons).filter(|&c| !is_pivot[c]).collect();
    let k = free_cols.len();
    let mut free_bounds = vec![0usize; k];
    for (idx, &col) in free_cols.iter().enumerate() {
        let mut bound = usize::MAX;
        for &counter in &buttons[col] {
            bound = bound.min(joltages[counter]);
        }
        free_bounds[idx] = bound;
    }
    let mut best: Option<i64> = None;
    let mut free_values = vec![0i64; k];
    fn dfs(
        idx: usize,
        free_cols: &[usize],
        is_pivot: &[bool],
        mat: &[Vec<Fraction>],
        free_bounds: &[usize],
        free_values: &mut Vec<i64>,
        best: &mut Option<i64>,
    ) {
        if idx == free_cols.len() {
            let m = is_pivot.len();
            let mut x = vec![Fraction::from_i64(0); m];
            for (fv_idx, &col) in free_cols.iter().enumerate() {
                x[col] = Fraction::from_i64(free_values[fv_idx]);
            }
            for row in 0..mat.len() {
                let mut pivot_col = None;
                for c in 0..m {
                    if mat[row][c].num == 1 && mat[row][c].den == 1 {
                        pivot_col = Some(c);
                        break;
                    }
                }
                if let Some(pc) = pivot_col {
                    let mut val = mat[row][m].clone();
                    for c in 0..m {
                        if c != pc && !mat[row][c].is_zero() {
                            val = val - mat[row][c].clone() * x[c].clone();
                        }
                    }
                    if !val.is_integer() || val.num < 0 {
                        return;
                    }
                    x[pc] = val;
                } else {
                    let mut lhs = Fraction::from_i64(0);
                    for c in 0..m {
                        if !mat[row][c].is_zero() {
                            lhs = lhs + mat[row][c].clone() * x[c].clone();
                        }
                    }
                    if lhs != mat[row][m] {
                        return;
                    }
                }
            }
            let mut total = 0i64;
            for v in x {
                if !v.is_integer() {
                    return;
                }
                let iv = v.to_i64().unwrap();
                if iv < 0 {
                    return;
                }
                total += iv;
            }
            if best.is_none() || total < best.unwrap() {
                *best = Some(total);
            }
            return;
        }
        let col_idx = idx;
        let ub = free_bounds[col_idx] as i64;
        for val in 0..=ub {
            if let Some(b) = *best {
                if (free_values.iter().take(idx).map(|&v| v).sum::<i64>() + val) >= b {
                    break;
                }
            }
            free_values[col_idx] = val;
            dfs(
                idx + 1,
                free_cols,
                is_pivot,
                mat,
                free_bounds,
                free_values,
                best,
            );
        }
    }
    dfs(
        0,
        &free_cols,
        &is_pivot,
        &mat,
        &free_bounds,
        &mut free_values,
        &mut best,
    );
    best.map(|v| v as usize)
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
    let debug = std::env::var("DEBUG_DAY10").is_ok();
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(idx, line)| {
            let (_, buttons, joltages) = parse_machine(line);
            let res = solve_machine_joltage(&joltages, &buttons);
            if debug {
                eprintln!("{idx}: {:?}", res);
            }
            res.unwrap_or(0)
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
