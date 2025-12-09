use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let r = self.find(i);
            *counts.entry(r).or_insert(0) += 1;
        }
        counts.values().cloned().collect()
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Vec<i64> = line
                .split(',')
                .map(|s| s.trim().parse::<i64>().expect("invalid number"))
                .collect();
            Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> u128 {
    const EDGES_TO_ADD: usize = 1000;
    let points = parse_points(input);
    let n = points.len();
    if n == 0 {
        return 0;
    }

    let mut edges: Vec<(u64, usize, usize)> = Vec::with_capacity(n.saturating_mul(n) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist2 = (dx * dx + dy * dy + dz * dz) as u64;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut uf = UnionFind::new(n);
    for &( _d, a, b) in edges.iter().take(EDGES_TO_ADD) {
        uf.union(a, b);
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by_key(|&s| Reverse(s));
    while sizes.len() < 3 {
        sizes.push(1);
    }
    let product = sizes.iter().take(3).fold(1u128, |acc, &s| acc * s as u128);
    product
}

fn solve_part2(input: &str) -> u128 {
    let points = parse_points(input);
    let n = points.len();
    if n == 0 {
        return 0;
    }

    let mut edges: Vec<(u64, usize, usize)> = Vec::with_capacity(n.saturating_mul(n) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist2 = (dx * dx + dy * dy + dz * dz) as u64;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut uf = UnionFind::new(n);
    let mut last_edge = None;
    
    for &(_d, a, b) in edges.iter() {
        let root_a = uf.find(a);
        let root_b = uf.find(b);
        if root_a != root_b {
            uf.union(a, b);
            last_edge = Some((a, b));
            
            let root = uf.find(a);
            if uf.size[root] == n {
                break;
            }
        }
    }

    if let Some((a, b)) = last_edge {
        (points[a].x * points[b].x) as u128
    } else {
        0
    }
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
