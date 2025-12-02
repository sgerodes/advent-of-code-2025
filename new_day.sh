#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <day-number>"
  echo "Example: $0 3"
  exit 1
fi

DAY_NUM="$1"
if ! [[ "$DAY_NUM" =~ ^[0-9]+$ ]]; then
  echo "Day number must be a positive integer"
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DAYS_DIR="$ROOT_DIR/days"
DAY_DIR="$DAYS_DIR/day$DAY_NUM"

if [[ -e "$DAY_DIR" ]]; then
  echo "Directory '$DAY_DIR' already exists; nothing to do."
  exit 0
fi

mkdir -p "$DAY_DIR/src"

cat > "$DAY_DIR/Cargo.toml" <<EOF
[package]
name = "day$DAY_NUM"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

cat > "$DAY_DIR/src/main.rs" <<'EOF'
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    println!("Day solution not implemented yet");
}
EOF

cat > "$DAY_DIR/README.md" <<EOF
### Day $DAY_NUM

EOF

touch "$DAY_DIR/input.txt"

echo "Created day$DAY_NUM in '$DAY_DIR'."
echo "Add \"days/day$DAY_NUM\" to your root Cargo.toml workspace members"

