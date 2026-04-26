# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Advent of Code 2023 solutions in Rust. Cargo workspace with 25 day crates plus a shared `common` crate. Rust edition 2024.

## Build and Run Commands

```bash
# Build all days
cargo build --release

# Run a specific day (requires input file as argument)
cargo run -p dayNN -- dayNN/dayNN.txt

# Run with part 2 feature enabled
cargo run -p dayNN --features part2 -- dayNN/dayNN.txt

# Build and run a specific day with release optimizations
cargo run --release -p dayNN -- dayNN/dayNN.txt

# Run tests for the common crate (the only crate with tests)
cargo test -p common
```

## Architecture

### Workspace Structure

- Root `Cargo.toml` defines the workspace and shared `[workspace.package]` metadata (`version`, `authors`, `description`). Each day crate inherits these via `version.workspace = true`, etc.
- 26 members: `common` plus `day01`..`day25`. Each day is an independent binary crate in `dayXX/`.
- Edition is `2024` for every crate (workspace and day crates).

### Input Data Location

Each day directory contains input files:

- `dayXX/dayXX.txt` - actual puzzle input
- `dayXX/dayXX-example.txt` - example/test input
- All solutions require the input file path as the first command-line argument.

### Common Utilities (`common` crate)

Located in `common/src/load.rs`. All loaders read the path from `args[1]` and return `Result<_, String>`:

- `load::string()` - entire file as a `String`
- `load::lines()` - `Vec<String>` of lines
- `load::comma_separated_values()` - splits the entire file by `,` and trims whitespace
- `load::map()` - 2D `Vec<Vec<char>>`, one row per line
- `load::numbers_map()` - 2D `Vec<Vec<i32>>` of single digits, non-digit chars silently ignored

The `common` crate is the only one with unit tests.

### Part 1 vs Part 2

Days use Cargo features for part switching:

- Feature `part2` defined in each day's `Cargo.toml`
- Conditional compilation via `cfg!(feature="part2")` (runtime branch) or `#[cfg(feature = "part2")]` / `#[cfg(not(feature = "part2"))]` (compile-time blocks)
- Same source file handles both parts
- Example: `const MAX_RUN: usize = if cfg!(feature="part2") { 10 } else { 3 };`

### Standardized Output

Every day's `main` follows the same output convention - preserve it when editing:

```rust
println!("=== Day N, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });
// ... compute ...
println!("Result: {}", answer);
```

The header line uses `=== Day N, part X ===` (with the equals signs and spaces) and the answer line is always `Result: <value>`.

### README.md Conventions

`README.md` records each day's commentary plus a results table of the form:

```markdown
| Part | Answer |
|-----:|-------:|
|    1 |  ...   |
|    2 |  ...   |
```

Column widths vary by day to fit the answer, and both columns are right-aligned (`-----:`). **Ignore markdown linter warnings about table column alignment / padding consistency** - the formatting is intentional and column widths are sized per-table.

### Common Patterns

- Input parsing via `common::load` functions
- Solution output printed to stdout in the standardized format above
- Heavy use of iterators and functional patterns
- Pathfinding problems (days 10, 17, 21, 23) use custom implementations rather than external crates
- `regex` is used by a few days (e.g. day02, day15, day20); it is added per-crate, not workspace-wide
