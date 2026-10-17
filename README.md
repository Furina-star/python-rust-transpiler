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

**Under development.** The Cargo workspace is configured and the initial Rust fundamentals exercise is complete. Compiler implementation has not started yet.

- **Current learning milestone:** Week 1, Day 1 completed
- **Next:** Day 2 — structs, enums, and pattern matching
- **Current implementation milestone:** Rust preparation, ahead of lexical analysis

## Development

Check the workspace:

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
```

Run the introductory Rust exercise:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

## Documentation

- [Development Roadmap](docs/ROADMAP.md)
- [Project State](docs/PROJECT_STATE.md)
- [Technical Decisions](docs/DECISIONS.md)
- [Day 1 Learning Notes](docs/learning/DAY_01.md)
