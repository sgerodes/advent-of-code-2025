use std::io::{self, Read};

#[derive(Clone)]
struct Orientation {
    w: usize,
    h: usize,
    rows: Vec<u64>,
    area: usize,
}

#[derive(Clone)]
struct Shape {
    area: usize,
    orientations: Vec<Orientation>,
}

fn rotate(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut out = vec![vec![false; h]; w];
    for y in 0..h {
        for x in 0..w {
            out[x][h - 1 - y] = grid[y][x];
        }
    }
    out
}

fn flip(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn normalize(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut top = h;
    let mut left = w;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] {
                if y < top {
                    top = y;
                }
                if x < left {
                    left = x;
                }
            }
        }
    }
    let mut trimmed = vec![vec![false; w - left]; h - top];
    for y in top..h {
        for x in left..w {
            trimmed[y - top][x - left] = grid[y][x];
        }
    }
    trimmed
}

fn unique_orientations(grid: &[Vec<bool>]) -> Vec<Orientation> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    let mut cur = grid.to_vec();
    for _ in 0..4 {
        for variant in [cur.clone(), flip(&cur)] {
            let norm = normalize(&variant);
            let key = norm
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|&b| if b { '1' } else { '0' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>();
            if seen.insert(key) {
                let h = norm.len();
                let w = norm[0].len();
                let mut rows = Vec::with_capacity(h);
                let mut area = 0usize;
                for y in 0..h {
                    let mut mask = 0u64;
                    for x in 0..w {
                        if norm[y][x] {
                            mask |= 1u64 << x;
                            area += 1;
                        }
                    }
                    rows.push(mask);
                }
                result.push(Orientation { w, h, rows, area });
            }
        }
        cur = rotate(&cur);
    }
    result
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let mut lines = input.lines().peekable();
    let mut shapes = Vec::new();
    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() {
            lines.next();
            break;
        }
        if !line.ends_with(':') {
            break;
        }
        lines.next();
        let mut grid_lines = Vec::new();
        while let Some(l) = lines.peek() {
            let l = l.trim_end();
            if l.is_empty() {
                break;
            }
            if l.chars().all(|c| c.is_digit(10) || c == 'x' || c == ':' || c == ' ') {
                break;
            }
            grid_lines.push(l.to_string());
            lines.next();
        }
        let grid: Vec<Vec<bool>> = grid_lines
            .iter()
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect();
        let orientations = unique_orientations(&grid);
        let area = orientations[0].area;
        shapes.push(Shape { area, orientations });
        if let Some(l) = lines.peek() {
            if l.trim().is_empty() {
                lines.next();
            }
        }
    }

    let mut regions = Vec::new();
    for line in lines {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let (dims, rest) = l.split_once(':').expect("region format");
        let (w_str, h_str) = dims.split_once('x').expect("dims format");
        let w: usize = w_str.parse().expect("width");
        let h: usize = h_str.parse().expect("height");
        let counts: Vec<usize> = rest
            .split_whitespace()
            .map(|s| s.parse().expect("count"))
            .collect();
        regions.push((w, h, counts));
    }

    (shapes, regions)
}

fn can_fit_region(w: usize, h: usize, counts: &[usize], shapes: &[Shape]) -> bool {
    if w < 1 || h < 1 {
        return false;
    }
    let total_area: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| c * shapes[i].area)
        .sum();
    if total_area > w * h {
        return false;
    }
    if (w < 3 || h < 3) && counts.iter().any(|&c| c > 0) {
        for (idx, &c) in counts.iter().enumerate() {
            if c == 0 {
                continue;
            }
            let mut fits_small = false;
            for ori in &shapes[idx].orientations {
                if ori.w <= w && ori.h <= h {
                    fits_small = true;
                    break;
                }
            }
            if !fits_small {
                return false;
            }
        }
    }

    let mut pieces = Vec::new();
    for (idx, &c) in counts.iter().enumerate() {
        for _ in 0..c {
            pieces.push(idx);
        }
    }
    pieces.sort_by_key(|&i| std::cmp::Reverse(shapes[i].area));

    let mut grid = vec![0u64; h];
    let row_mask_full = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };

    for &sid in &pieces {
        let mut placed = false;
        let shape = &shapes[sid];
        'outer: for ori in &shape.orientations {
            if ori.w > w || ori.h > h {
                continue;
            }
            let max_y = h + 1 - ori.h;
            let max_x = w + 1 - ori.w;
            for y in 0..max_y {
                for x in 0..max_x {
                    let mut overlap = false;
                    for (row_idx, &mask) in ori.rows.iter().enumerate() {
                        let shifted = mask << x;
                        if shifted & !row_mask_full != 0 {
                            overlap = true;
                            break;
                        }
                        if (grid[y + row_idx] & shifted) != 0 {
                            overlap = true;
                            break;
                        }
                    }
                    if overlap {
                        continue;
                    }
                    for (row_idx, &mask) in ori.rows.iter().enumerate() {
                        grid[y + row_idx] |= mask << x;
                    }
                    placed = true;
                    break 'outer;
                }
            }
        }
        if !placed {
            return false;
        }
    }

    true
}

fn solve_part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    let mut ok = 0usize;
    for (w, h, counts) in regions {
        if counts.len() != shapes.len() {
            continue;
        }
        if can_fit_region(w, h, &counts, &shapes) {
            ok += 1;
        }
    }
    ok
}

fn solve_part2(_input: &str) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || (args[1] != "part1" && args[1] != "part2") {
        eprintln!("Usage: {} <part1|part2>", args[0]);
        std::process::exit(1);
    }

    let ans = if args[1] == "part1" {
        solve_part1(&input)
    } else {
        solve_part2(&input)
    };
    println!("{ans}");
}
