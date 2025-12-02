### Advent of Code 2025 – Overview

This repository contains my solutions for **Advent of Code 2025**.  
Each day lives as its own Cargo crate under `challenges/` with the puzzle description, input, and Rust solution.

### How to run days

- **From the repo root** (preferred):
  - Day 1:  
    `cargo run -p day1 --release < days/day1/input.txt`
- **From within a specific day crate**:
  - Day 1:  
    `cd days/day1 && cargo run --release < input.txt`

| Day | Description (AoC) | Local Description                  | Crate / Entry Point                |
| --- | ----------------- |------------------------------------|------------------------------------|
| 1 | [Secret Entrance](https://adventofcode.com/2025/day/1) | [`challenge`](days/day1/README.md) | [`main.rs`](days/day1/src/main.rs) |
