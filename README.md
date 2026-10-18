# Python-to-Rust Transpiler

A learning-focused compiler engineering project written in Rust.

## Overview

This project aims to translate a restricted, statically typed Python-like language into equivalent Rust source code.

The primary goal is to explore lexical analysis, parsing, abstract syntax trees (ASTs), semantic analysis, and code generation.

**Note:** The project does not aim to provide full Python compatibility.

## Planned Language Features

- Integer arithmetic
- Typed function declarations
- Variables and assignments
- Return statements
- Conditional statements
- While loops

These features are planned; they have not been implemented yet.

## Planned Architecture

```text
Source Code
    ↓
Lexer
    ↓
Parser
    ↓
Abstract Syntax Tree
    ↓
Semantic Analysis
    ↓
Rust Code Generator
    ↓
Generated Rust Source
```

## Project Status

**Under development.** The Cargo workspace contains one library crate, `python-rust-transpiler`, and three runnable Rust practice examples. The compiler source files are placeholders; no transpilation API or CLI is available yet.

- **Current learning milestone:** Week 1, Day 3 completed — vectors, strings, loops, and iterators understood and practiced in a working example
- **Next:** Day 4 — `Option`, `Result`, and basic error handling
- **Current implementation milestone:** Rust preparation, ahead of lexical analysis

See [Project State](docs/PROJECT_STATE.md) for verification results and remaining follow-ups.

## Repository Layout

```text
Cargo.toml                  Workspace manifest
crates/transpiler/
    Cargo.toml              Library crate manifest
    src/                    Placeholder compiler source files
    examples/               Runnable Rust learning exercises
docs/                       Project documentation
```

## Development

Use a Rust toolchain with Cargo and support for the Rust 2024 edition and workspace resolver `3`, as configured in the manifests. The formatting command also requires `rustfmt`. Run the following commands from the repository root.

Check the library and examples, run the test harness, and check formatting:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
```

There are currently no unit or documentation tests. A successful test command verifies the test harness builds and runs; it does not validate compiler behavior. The examples currently emit warnings about unused fields and variants. See [Project State](docs/PROJECT_STATE.md) for the latest formatting result.

Run the introductory Rust exercise:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

Run the Day 2 practice example:

```bash
cargo run -p python-rust-transpiler --example rust_structs
```

Run the Day 3 practice example:

```bash
cargo run -p python-rust-transpiler --example rust_vectors
```

The basics example prints `Ahri's remaining health: 1000`. The Day 2 example prints the player's health and experience, followed by `Faker is respawning in 5 seconds...`.

The Day 3 example practices vector growth and indexing, `String` and `&str`, `while` and `for` loops, and `iter()`, `iter_mut()`, and `into_iter()`. It prints total damage `1000`, adds `50` to each value through mutable iteration, and prints `[150, 250, 350, 450]` with total damage `1200`.

## Documentation

- [Development Roadmap](docs/ROADMAP.md)
- [Project State](docs/PROJECT_STATE.md)
- [Technical Decisions](docs/DECISIONS.md)
