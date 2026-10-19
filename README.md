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

**Early development.** The Cargo workspace is set up, with four runnable Rust practice examples. Compiler implementation has not started.

- **Completed:** Rust basics, structs, enums, collections, iterators, and error handling.
- **Next:** Ownership and borrowing, followed by lexical analysis.
- **Availability:** No transpilation API or CLI yet.

See [Project State](docs/PROJECT_STATE.md) for detailed progress and verification results.

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

Use a Rust toolchain that supports the 2024 edition, with Cargo and `rustfmt`. Run commands from the repository root.

Check the project:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo fmt --all -- --check
```

The test harness currently contains no tests. Known warnings are listed in [Project State](docs/PROJECT_STATE.md).

Run a practice example:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

Replace `rust_basics` with any example below:

| Example | Topics |
| --- | --- |
| [rust_basics](crates/transpiler/examples/rust_basics.rs) | Variables, types, functions, and mutability |
| [rust_structs](crates/transpiler/examples/rust_structs.rs) | Structs, enums, and pattern matching |
| [rust_vectors](crates/transpiler/examples/rust_vectors.rs) | Vectors, strings, loops, and iterators |
| [rust_options](crates/transpiler/examples/rust_options.rs) | `Option`, `Result`, and error handling |

## Documentation

- [Development Roadmap](docs/ROADMAP.md)
- [Project State](docs/PROJECT_STATE.md)
- [Technical Decisions](docs/DECISIONS.md)
