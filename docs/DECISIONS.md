# Technical Decisions

## Decision 001: Implementation Language

Use Rust for the transpiler. The library uses the 2024 edition.

## Decision 002: Initial Scope

Target a restricted, statically typed Python-like language. Full Python compatibility is out of scope.

## Decision 003: Architecture

Develop a standalone library before adding CLI and procedural macro interfaces. The intended pipeline is lexing, parsing into an AST, semantic validation, and Rust code generation.

## Decision 004: Parsing Strategy

Use a hand-written lexer and a recursive-descent parser. The lexer is implemented; parsing begins in Phase 2.

## Decision 005: Development Approach

Implement and test one component at a time. Keep completed milestones in the [roadmap](ROADMAP.md) and measured verification results in [Project State](PROJECT_STATE.md).

## Decision 006: Indentation and Line Continuations

Use spaces for indentation and `Newline`, `Indent`, and `Dedent` for layout. Dedents must match a recorded indentation level. Ignore blank lines and comments; suppress layout inside parentheses.

The lexer records indentation increases without checking whether a block is allowed. The parser is intended to reject unexpected indentation; that validation is not implemented yet.

## Decision 007: Initial Lexical Scope

Use ASCII identifiers and decimal `i32` integer literals. Match keywords and booleans exactly and case-sensitively after reading a complete identifier. Type names remain identifiers for later stages to interpret.

Signs are separate tokens. Digit sequences must fit in `i32`, so the magnitude in `-2147483648` currently overflows. Wider integer support is deferred. Compound assignment is also deferred: operators such as `+=` have no dedicated token or implemented semantics.

## Decision 008: Lexer Diagnostics

Return the first detected error as `Diagnostic { position, message }` from the core lexer. Use line and column information for scanning, indentation, and parenthesis errors. `Display` formats diagnostics as `Line N, Column M: message`. Recovery and multiple-error reporting are deferred.

## Decision 009: Source Positions

Keep token values in `Token` and attach locations through `LocatedToken`. `SourcePosition` stores one-based lines and character columns, not byte offsets or full spans. Ordinary tokens use their starting positions; synthetic layout tokens follow the [layout contract](ARCHITECTURE.md#layout-and-end-of-input).

## Decision 010: Final Newline and EOF

For input containing code, emit a final `Newline`, close remaining indentation levels with `Dedent`, then emit `Eof`. Input without code emits only `Eof`. This gives the parser explicit statement, block, and input boundaries regardless of a trailing physical line ending.

## Decision 011: Backward-Compatible API Signatures

Retain `tokenize(&str) -> Result<Vec<Token>, String>` and `tokenize_with_positions(&str) -> Result<Vec<LocatedToken>, String>`. Both delegate to `tokenize_with_diagnostics(&str) -> Result<Vec<LocatedToken>, Diagnostic>`.

Compatibility applies to function signatures, not identical output: callers must handle the added final `Newline` and `Eof` tokens, and structural error strings now include columns.

[Project overview](../README.md) · [Architecture](ARCHITECTURE.md)
