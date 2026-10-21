# Planned Architecture

## Current Status

The Cargo workspace contains one library crate, `crates/transpiler`. Rust fundamentals are complete, and six runnable examples live in `crates/transpiler/examples/`. The library entry point and compiler source files are placeholders; the pipeline below is planned.

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

## Planned Components

Paths below are relative to `crates/transpiler/src/`.

| File | Responsibility |
| --- | --- |
| `lib.rs` | Expose the library API and connect compiler stages |
| `token.rs` | Define token types shared by the lexer and parser |
| `lexer.rs` | Convert source text into tokens, including indentation handling |
| `parser.rs` | Use recursive descent to build an AST from tokens |
| `ast.rs` | Define representations for expressions, statements, and functions |
| `semantic.rs` | Validate names, types, and scopes using symbol tables |
| `codegen.rs` | Generate Rust source from the validated AST |

## Development Order

Build and test the standalone library before adding CLI and procedural macro interfaces. The [roadmap](ROADMAP.md) starts with lexical analysis, then parsing, initial code generation, and semantic analysis. In the intended runtime pipeline, semantic analysis precedes code generation.

The next step is to define compiler token types and implement arithmetic tokenization with lexer tests. The `Token` enum in the integration example is local practice code; it is not yet part of the library.

## Related Documentation

- [Project State](PROJECT_STATE.md)
- [Development Roadmap](ROADMAP.md)
- [Technical Decisions](DECISIONS.md)
