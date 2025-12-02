### Advent of Code 2025 – Overview

This repository contains my solutions for **Advent of Code 2025**.  
Each day lives as its own Cargo crate under `challenges/` with the puzzle description, input, and Rust solution.

### How to run a day

- **From the repo root**:
  - Change into the day directory (note the space in the name):  
    `cd "challenges/day 1"`
  - Run with your puzzle input (stored as `input.txt` or from stdin):  
    `cargo run --release < input.txt`
- **For other days** (once added), use the same pattern:  
  `cd "challenges/day 1" && cargo run --release < input.txt`

| Day | Description (AoC) | Local Description                           | Crate / Entry Point                     |
| --- | ----------------- |---------------------------------------------| ----------------------------------------|
| 1 | [Secret Entrance](https://adventofcode.com/2025/day/1) | [`challenge`](challenges/day%201/README.md) | [`main.rs`](challenges/day%201/src/main.rs) |
