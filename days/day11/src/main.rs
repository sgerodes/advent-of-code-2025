use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((src, rest)) = line.split_once(':') {
            let src = src.trim().to_string();
            let targets: Vec<String> = rest
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            graph.insert(src, targets);
        }
    }
    graph
}

fn count_paths_between(
    start: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, String), u128>,
) -> u128 {
    if let Some(v) = memo.get(&(start.to_string(), target.to_string())) {
        return *v;
    }
    if start == target {
        memo.insert((start.to_string(), target.to_string()), 1);
        return 1;
    }
    let mut total = 0u128;
    if let Some(nexts) = graph.get(start) {
        for n in nexts {
            total = total.saturating_add(count_paths_between(n, target, graph, memo));
        }
    }
    memo.insert((start.to_string(), target.to_string()), total);
    total
}

fn count_paths_excluding(
    start: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    forbidden: &HashSet<&str>,
    memo: &mut HashMap<(String, String, Vec<String>), u128>,
) -> u128 {
    if forbidden.contains(start) && start != target {
        return 0;
    }
    let mut key_forb: Vec<String> = forbidden.iter().map(|s| (*s).to_string()).collect();
    key_forb.sort();
    let key = (start.to_string(), target.to_string(), key_forb.clone());
    if let Some(v) = memo.get(&key) {
        return *v;
    }
    if start == target {
        memo.insert(key, 1);
        return 1;
    }
    let mut total = 0u128;
    if let Some(nexts) = graph.get(start) {
        for n in nexts {
            if forbidden.contains(n.as_str()) {
                continue;
            }
            total = total.saturating_add(count_paths_excluding(
                n,
                target,
                graph,
                forbidden,
                memo,
            ));
        }
    }
    memo.insert(key, total);
    total
}

fn solve_part1(input: &str) -> u128 {
    let graph = parse_graph(input);
    let mut memo = HashMap::new();
    count_paths_between("you", "out", &graph, &mut memo)
}

fn solve_part2(input: &str) -> u128 {
    let graph = parse_graph(input);
    let mut memo_any = HashMap::new();
    let mut memo_excl = HashMap::new();
    let via_dac_fft = count_paths_excluding(
        "svr",
        "dac",
        &graph,
        &HashSet::from(["fft"]),
        &mut memo_excl,
    ) * count_paths_between("dac", "fft", &graph, &mut memo_any)
        * count_paths_between("fft", "out", &graph, &mut memo_any);
    memo_excl.clear();
    let via_fft_dac = count_paths_excluding(
        "svr",
        "fft",
        &graph,
        &HashSet::from(["dac"]),
        &mut memo_excl,
    ) * count_paths_between("fft", "dac", &graph, &mut memo_any)
        * count_paths_between("dac", "out", &graph, &mut memo_any);
    via_dac_fft + via_fft_dac
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
    let answer = if args[1] == "part1" {
        solve_part1(&input)
    } else {
        solve_part2(&input)
    };
    println!("{answer}");
}
