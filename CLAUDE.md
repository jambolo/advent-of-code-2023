# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## General Instructions for Claude

### Token Discipline

- Be concise by default.
- No explanations unless explicitly requested.
- No restating the question.
- No summaries at the end.
- Use bullet points only when clarity improves.
- Prefer short sentences.
- Assume reader is expert.

### Output Rules

- Answer the question directly.
- Do not add context, background, or alternatives unless asked.
- If uncertain, say "unknown" or ask one clarifying question.

### Code

- Output code only, no commentary.
- Prefer minimal, idiomatic solutions.
- Limit comments to very brief descriptions of what the code does. Do not describe why changes were made.

### Interaction

- Ask at most one clarifying question.
- Never suggest next steps unless requested.

## Project Overview

Advent of Code 2023 solutions in Rust. Cargo workspace with 25 day crates plus a shared `common` crate.

## Build and Run Commands

```bash
# Build all days
cargo build --release

# Run a specific day (requires input file as argument)
cargo run -p day01 -- day01/day1.txt

# Run with part 2 feature enabled
cargo run -p day01 --features part2 -- day01/day1.txt

# Build and run specific day with release optimizations
cargo build -p day17 --release --features part2
./target/release/day17 day17/day17.txt
```

## Architecture

### Workspace Structure
- Root `Cargo.toml` defines workspace with 28 members (25 days + common)
- Each day is an independent binary crate in `dayXX/` directory
- Shared utilities in `common` crate

### Input Data Location
Each day directory contains input files:
- `dayXX/dayXX.txt` - actual puzzle input
- `dayXX/dayXX-example.txt` - example/test input
- All solutions require input file path as command-line argument

### Common Utilities (`common` crate)
Located in `common/src/load.rs`, provides input parsing:
- `load::lines()` - loads lines into `Vec<String>`
- `load::comma_separated_values()` - splits by commas
- `load::map()` - 2D char array (`Vec<Vec<char>>`)
- `load::numbers_map()` - 2D number array
- All functions read from `args[1]` path

### Part 1 vs Part 2
Days use Cargo features for part switching:
- Feature `part2` defined in each day's `Cargo.toml`
- Conditional compilation: `cfg!(feature="part2")`
- Same source file handles both parts
- Example: `const MAX_RUN: usize = if cfg!(feature="part2") { 10 } else { 3 };`

### Common Patterns
- Days typically print which part is running: `println!("Day X, part {}", if cfg!(feature="part2") { "2" } else { "1" })`
- Input parsing via `common::load` functions
- Solution output printed to stdout
- Heavy use of iterators and functional patterns
- Pathfinding problems (days 10, 17, 21, 23) use custom implementations rather than external crates
