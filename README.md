# Python-to-Rust Transpiler

A Python-to-Rust transpiler written in Rust, targeting a restricted, statically typed Python-like language.

## Overview

This project aims to translate a restricted, statically typed Python-like language into equivalent Rust source code.

The planned compiler pipeline combines lexical analysis, recursive-descent parsing, semantic validation, and Rust code generation.

**Note:** The project does not aim to provide full Python compatibility.

## Planned Language Features

- Integer arithmetic
- Typed function declarations
- Variables and assignments
- Return statements
- Conditional statements
- While loops

These language features are planned. Initial tokenization is implemented; parsing, semantic analysis, and Rust code generation are still pending.

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

## Repository Layout

```text
Cargo.toml                  Workspace manifest
crates/transpiler/
    Cargo.toml              Library crate manifest
    src/                    Token types, initial lexer, and placeholders for later stages
    examples/               Standalone Rust examples
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

The test harness contains 13 lexer unit tests. Current limitations and verification results are listed in [Project State](docs/PROJECT_STATE.md).

## Lexer API

```rust
use python_rust_transpiler::{lexer::tokenize, token::Token};

let tokens = tokenize("return total + 12").unwrap();
assert_eq!(tokens, vec![
    Token::Return,
    Token::Identifier(String::from("total")),
    Token::Plus,
    Token::Integer(12),
]);
```

`tokenize(&str)` returns `Result<Vec<Token>, String>`. It groups ASCII digits into `i32` values and reads full identifiers before matching the exact keywords `def` and `return`; `defense` and `returnValue` remain identifiers. Identifiers start with an ASCII letter or `_`, followed by ASCII letters, digits, or `_`.

Only ordinary spaces are skipped. Integer overflow and unsupported characters return errors. Parentheses, colons, and commas have token variants but are not recognized yet; tabs, newlines, and indentation are also unsupported. A minus sign is emitted as a separate token.

## Examples

Standalone Rust examples are available in [`crates/transpiler/examples/`](crates/transpiler/examples/). Run one with:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

These examples demonstrate Rust language features independently of the compiler API. For source tokenization, use the lexer example above.

## Documentation

- [Development Roadmap](docs/ROADMAP.md)
- [Project State](docs/PROJECT_STATE.md)
- [Planned Architecture](docs/ARCHITECTURE.md)
- [Technical Decisions](docs/DECISIONS.md)
