<h1 align="center">Python-to-Rust Transpiler</h1>

<p align="center">
  <a href="https://github.com/Furina-star/python-rust-transpiler/actions/workflows/ci.yml">
    <img src="https://github.com/Furina-star/python-rust-transpiler/actions/workflows/ci.yml/badge.svg" alt="Rust CI">
  </a>
  <a href="crates/transpiler/Cargo.toml">
    <img src="https://img.shields.io/badge/Rust-2024_edition-orange?logo=rust" alt="Rust 2024 edition">
  </a>
  <a href="docs/ROADMAP.md">
    <img src="https://img.shields.io/badge/status-in_development-yellow" alt="Status: in development">
  </a>
</p>

<p align="center">
  A Python-to-Rust transpiler written in Rust, targeting a restricted, statically typed Python-like language.
</p>

## Overview

This project aims to translate a restricted, statically typed Python-like language into equivalent Rust source code.

The planned compiler pipeline combines lexical analysis, recursive-descent parsing, semantic validation, and Rust code generation.

Phase 1: Lexical Analysis is complete for the initial language scope. The library provides a tested lexer with indentation handling and error locations. Syntax analysis is the next milestone.

**Note:** The project does not aim to provide full Python compatibility.

## Planned Language Features

- Integer arithmetic
- Boolean values, comparisons, and logical expressions
- Typed function declarations
- Variables and assignments
- Return statements
- Conditional statements
- While loops

The lexer recognizes the tokens needed for these features. Parsing, semantic analysis, and Rust code generation are still pending, so source programs cannot yet be transpiled or executed.

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
    src/                    Token types, lexer, and placeholders for later stages
        lexer/tests.rs      Lexer unit tests
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

The test suite contains 65 lexer unit tests covering token recognition, indentation, line continuations, comments, and diagnostics, plus two integration tests for tokenization and error reporting through the public API. Current limitations and verification results are listed in [Project State](docs/PROJECT_STATE.md).

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

`tokenize(&str)` returns `Result<Vec<Token>, String>`. It recognizes `i32` integer literals, booleans, ASCII identifiers, keywords, arithmetic and comparison operators, assignment, punctuation, and return type arrows. Keywords match complete identifiers; names such as `defense` and `returnValue` remain identifiers.

The lexer emits `Newline`, `Indent`, and `Dedent` tokens for statement and block boundaries. It accepts Unix and Windows line endings, skips blank lines and comments, and supports line continuations inside parentheses. Open indentation levels close at the end of input.

Invalid characters, integer overflow, tabs in code, and bare carriage returns produce errors with line and column numbers. Inconsistent indentation and unbalanced parentheses produce errors with line numbers. Tokens do not yet carry source spans. See [Supported Lexer Behavior](docs/PROJECT_STATE.md#supported-lexer-behavior) for the complete token set and limitations.

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

## References & Acknowledgments

- **Robert Nystrom — [*Crafting Interpreters*](https://craftinginterpreters.com/)**  
  An educational reference on programming language implementation. Its discussions of recursive descent parsing and tree-walk interpretation provide valuable examples of how recursion is applied in interpreters and language processing.

Recursion Lab is an independent educational project focused on understanding recursive algorithms and their applications. This acknowledgment does not imply affiliation with or endorsement by the author.
