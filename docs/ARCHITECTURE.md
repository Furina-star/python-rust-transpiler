# Planned Architecture

## Current Status

The Cargo workspace contains one library crate, `crates/transpiler`. The library entry point publicly exposes `lexer` and `token`. Phase 1: Lexical Analysis is complete for the initial language scope; the remaining pipeline stages are planned.

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
| `token.rs` | Define token types shared by the lexer and parser | `Token` enum for literals, identifiers, keywords, operators, punctuation, and layout; derives `Debug` and `PartialEq` |
| `lexer.rs` | Convert source text into tokens, including indentation handling | Complete for Phase 1, including comments, line continuations, and error locations |
| `lexer/tests.rs` | Verify token recognition, layout, and diagnostics | 65 unit tests, loaded through the lexer's test module |
| `parser.rs` | Use recursive descent to build an AST from tokens | Empty placeholder |
| `ast.rs` | Define representations for expressions, statements, and functions | Empty placeholder |
| `semantic.rs` | Validate names, types, and scopes using symbol tables | Empty placeholder |
| `codegen.rs` | Generate Rust source from the validated AST | Empty placeholder |

## Current Lexer

`lexer::tokenize(source: &str) -> Result<Vec<Token>, String>` processes source lines and delegates character scanning to the private `tokenize_line` function. The scanner uses lookahead to distinguish operators such as `-` and `->`, `/` and `//`, and `=` and `==`. It groups ASCII digits into `i32` values and reads complete identifiers before matching keywords and booleans. The identifier rules are `[A-Za-z_][A-Za-z0-9_]*`.

An indentation stack starts at zero spaces. Increased indentation emits `Indent`; a decrease must match an existing level and emits one `Dedent` per closed level. Remaining levels close at the end of input. Blank lines and comment-only lines are skipped, and inline comments are ignored.

A stack of opening-parenthesis line numbers tracks continuations and detects unmatched or unclosed parentheses. While parentheses remain open, subsequent lines do not introduce newline or indentation tokens. Outside continuations, `Newline` separates nonblank code lines. A trailing line ending does not add a final `Newline` token. Unix and Windows line endings produce equivalent tokens.

The private `ScanError` carries a one-based character column and a message. The public API adds the one-based source line and returns a string error. Scanning errors include line and column numbers; indentation and parenthesis errors include line numbers. Successful tokens do not carry source spans. Parsing, type validation, and evaluation remain responsibilities of later stages.

## Development Order

Build and test the standalone library before adding CLI and procedural macro interfaces. The [roadmap](ROADMAP.md) starts with lexical analysis, then parsing, initial code generation, and semantic analysis. In the intended runtime pipeline, semantic analysis precedes code generation.

The next step is to define the AST and implement expression, statement, and function parsing over the existing token stream.

The parser must accept `Newline`, `Dedent`, or end-of-input as appropriate statement boundaries. The lexer does not guarantee a final `Newline`: `x=1` ends after its integer token, and `if x:\n    y=1` ends with `Dedent` immediately after the final integer. Statement parsing must leave `Dedent` for block parsing to consume and detect end-of-input when the token stream is exhausted.

## Related Documentation

- [Project State](PROJECT_STATE.md)
- [Development Roadmap](ROADMAP.md)
- [Technical Decisions](DECISIONS.md)
