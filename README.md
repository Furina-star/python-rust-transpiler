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

Python-to-Rust Transpiler aims to translate a restricted, statically typed Python-like language into Rust source code. Full Python compatibility is outside its scope.

Phases 0, 1, and 1.5 are complete. The library provides lexical analysis, positioned tokens, and structured diagnostics. **Phase 2: Syntax Analysis is next.** Parsing, type checking, and Rust code generation are not implemented, so programs cannot yet be transpiled or executed.

## Supported Features

- Integer and boolean literals, ASCII identifiers, keywords, operators, and punctuation.
- Space-based indentation, comments, Unix and Windows line endings, and parenthesized line continuations.
- `Newline`, `Indent`, `Dedent`, and `Eof` tokens, with source positions and error diagnostics.

The intended language includes typed functions, variables, assignments, returns, conditionals, and while loops. The lexer recognizes their tokens; later phases will validate their syntax and meaning. See [Project State](docs/PROJECT_STATE.md) for limitations.

## Architecture

```text
Source -> Lexer -> Parser -> AST -> Semantic Analysis -> Rust Code Generator -> Rust Source
```

Only the lexer and its token and diagnostic types are implemented. See [Architecture](docs/ARCHITECTURE.md) for module responsibilities and API contracts.

## Repository Layout

| Path | Contents |
| --- | --- |
| `Cargo.toml` | Workspace with one library crate |
| `crates/transpiler/src/` | Lexer, tokens, diagnostics, and placeholder compiler files |
| `crates/transpiler/src/lexer/tests.rs` | Lexer unit tests |
| `crates/transpiler/tests/` | Public API integration tests |
| `crates/transpiler/examples/` | Six standalone Rust examples |
| `cli/`, `python_macro/` | Placeholder directories; not workspace crates |
| `docs/` | Status, roadmap, architecture, and decisions |

## Lexer API

```rust
use python_rust_transpiler::{lexer::tokenize, token::Token};

let tokens = tokenize("return total + 12").unwrap();
assert_eq!(tokens, vec![
    Token::Return,
    Token::Identifier(String::from("total")),
    Token::Plus,
    Token::Integer(12),
    Token::Newline,
    Token::Eof,
]);
```

Choose `tokenize` for token values, `tokenize_with_positions` for located tokens and string errors, or `tokenize_with_diagnostics` for located tokens and structured errors. All accept `&str`; their exact return types are listed in [Lexer APIs](docs/ARCHITECTURE.md#lexer-apis).

## Development

Use a current stable Rust toolchain with Cargo, `rustfmt`, and Clippy. The crate uses the 2024 edition. Run the same checks as CI from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --lib --tests -- -D warnings
cargo test --workspace
```

See [Verification](docs/PROJECT_STATE.md#verification) for current results.

The [standalone examples](crates/transpiler/examples/) demonstrate Rust features independently of the lexer. Run one with:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

## Documentation

- [Project State](docs/PROJECT_STATE.md): current functionality, limitations, and verification.
- [Development Roadmap](docs/ROADMAP.md): completed and planned phases.
- [Architecture](docs/ARCHITECTURE.md): modules, APIs, and token behavior.
- [Technical Decisions](docs/DECISIONS.md): design choices and deferred scope.

## References & Acknowledgments

- **Robert Nystrom — [*Crafting Interpreters*](https://craftinginterpreters.com/)**  
  An educational reference on programming language implementation. Its discussions of recursive descent parsing and tree-walk interpretation provide valuable examples of how recursion is applied in interpreters and language processing.

Python-to-Rust Transpiler is an independent project that aims to translate a restricted, statically typed Python-like language into Rust source code. This acknowledgment does not imply affiliation with or endorsement by Robert Nystrom.
