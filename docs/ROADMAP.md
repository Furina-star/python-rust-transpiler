# Development Roadmap

Phases 0, 1, and 1.5 are complete for the initial language scope. **Phase 2: Syntax Analysis is next.** Checked items represent implemented milestones; verification lives in [Project State](PROJECT_STATE.md).

## Phase 0: Project Initialization — Complete

- [x] Configure the Cargo workspace and Git repository.
- [x] Create initial documentation and verify the workspace builds.

## Phase 1: Lexical Analysis — Complete

- [x] Define tokens for integers, booleans, ASCII identifiers, and keywords.
- [x] Recognize arithmetic, assignment, comparison, punctuation, and arrow tokens.
- [x] Handle indentation, dedentation, comments, and blank lines.
- [x] Support Unix/Windows line endings and parenthesized continuations.
- [x] Detect invalid characters, integer literals, indentation, and parentheses.
- [x] Add lexer unit tests and public API integration tests.

## Phase 1.5: Lexer Preparation for Parsing — Complete

- [x] Add `SourcePosition` and `LocatedToken`.
- [x] Add structured `Diagnostic` errors with line and column information.
- [x] Expose positioned and diagnostic APIs alongside the original `tokenize` signature.
- [x] Emit a final `Newline` for code, close remaining blocks, and always emit `Eof`.
- [x] Test token positions, diagnostics, and end-of-input behavior.

## Phase 2: Syntax Analysis — Next

- [ ] Define AST types for expressions, statements, and typed functions.
- [ ] Implement recursive-descent expression parsing with precedence.
- [ ] Parse assignments, returns, conditionals, loops, and function bodies.
- [ ] Consume `Newline`, `Indent`, `Dedent`, and `Eof` according to the grammar.
- [ ] Reject unexpected indentation and report syntax errors with source positions.
- [ ] Add parser tests for valid and invalid syntax.

## Phase 3: Semantic Analysis

- [ ] Add symbol tables and validate declarations, names, types, and scopes.
- [ ] Integrate validation before code generation in the compilation pipeline.

## Phase 4: Code Generation

- [ ] Generate Rust expressions, statements, and functions from the AST.
- [ ] Compile and test generated code for the supported subset.

Phase numbers retain the development order; the intended runtime pipeline validates semantics before generating Rust.

## Phase 5: CLI and End-to-End Testing

- [ ] Add a CLI for source input and generated Rust output.
- [ ] Add end-to-end transpilation tests and user-facing diagnostics.

## Phase 6: Procedural Macros

- [ ] Create a macro crate after the library API stabilizes.
- [ ] Integrate the transpiler and test macro expansion.

Wider integers and compound assignment remain deferred; see [Technical Decisions](DECISIONS.md).

[Project overview](../README.md) · [Architecture](ARCHITECTURE.md)
