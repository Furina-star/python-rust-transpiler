# Planned Architecture

## Current Status

The Cargo workspace contains one library crate, `crates/transpiler`. The library entry point publicly exposes `lexer` and `token`. Initial lexical analysis is implemented; the remaining pipeline stages are planned.

## Compilation Pipeline

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

The input will be a restricted, statically typed Python-like language. The output will be Rust source code. Full Python compatibility is outside the initial scope.

## Components

Paths below are relative to `crates/transpiler/src/`.

| File | Responsibility | Current state |
| --- | --- | --- |
| `lib.rs` | Expose the library API and connect compiler stages | Exports `lexer` and `token` |
| `token.rs` | Define token types shared by the lexer and parser | Initial `Token` enum, deriving `Debug` and `PartialEq` |
| `lexer.rs` | Convert source text into tokens, including indentation handling | Integers, `+`/`-`, ASCII identifiers, `def`/`return`, and 13 unit tests; indentation pending |
| `parser.rs` | Use recursive descent to build an AST from tokens | Empty placeholder |
| `ast.rs` | Define representations for expressions, statements, and functions | Empty placeholder |
| `semantic.rs` | Validate names, types, and scopes using symbol tables | Empty placeholder |
| `codegen.rs` | Generate Rust source from the validated AST | Empty placeholder |

## Current Lexer

`lexer::tokenize(source: &str) -> Result<Vec<Token>, String>` scans a peekable character iterator. It groups consecutive ASCII digits and parses them as `i32`, propagating overflow errors as strings. For identifiers, it consumes the longest valid sequence before matching the exact keywords `def` and `return` (maximal munch). The identifier rules are `[A-Za-z_][A-Za-z0-9_]*`.

The lexer skips ordinary spaces and returns an error on the first unsupported character. It does not yet track source locations, emit newline or indentation tokens, or recognize punctuation. `LeftParen`, `RightParen`, `Colon`, and `Comma` exist in the token enum but are not yet emitted. Parsing and evaluation are outside the lexer.

## Development Order

Build and test the standalone library before adding CLI and procedural macro interfaces. The [roadmap](ROADMAP.md) starts with lexical analysis, then parsing, initial code generation, and semantic analysis. In the intended runtime pipeline, semantic analysis precedes code generation.

The next step is to recognize the defined punctuation tokens and test them, then add newline and indentation handling.

## Related Documentation

- [Project State](PROJECT_STATE.md)
- [Development Roadmap](ROADMAP.md)
- [Technical Decisions](DECISIONS.md)
