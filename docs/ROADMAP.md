# Development Roadmap

The workspace is initialized and the Day 1 through Day 4 practice examples run successfully. `Option`, `Result`, and basic error handling are complete; Day 5's ownership, borrowing, and review exercises are next. Compiler implementation has not started. See [Project State](PROJECT_STATE.md) for verification results and remaining warnings.

Checked items record completed milestones, not a guarantee that every repository check passes. The compiler phases below describe implementation order; the intended runtime pipeline places semantic analysis before code generation.

## Phase 0: Project Initialization

- [x] Configure Cargo workspace
- [x] Configure Git
- [x] Create initial documentation
- [x] Verify workspace builds

## Preparation: Rust Fundamentals (Week 1)

- [x] Day 1: Variables, mutability, basic types, functions, and return expressions
- [x] Day 2: Structs, enums, and pattern matching, including associated data (example built and run)
- [x] Day 3: `Vec`, `String`, loops, and iterators, including `iter`, `iter_mut`, and `into_iter` (example built and run)
- [x] Day 4: `Option`, `Result`, and basic error handling, including `?` propagation, `map_err`, and input validation (example built and run)
- [ ] Day 5: Ownership, borrowing, and review exercises

> These learning days are preparatory work, separate from the compiler implementation phases below.

## Phase 1: Lexical Analysis

- [ ] Define token types
- [ ] Implement arithmetic tokenization
- [ ] Recognize identifiers and keywords
- [ ] Handle indentation
- [ ] Write lexer tests

## Phase 2: Syntax Analysis

- [ ] Define the AST
- [ ] Implement expression parsing
- [ ] Implement statement parsing
- [ ] Implement function parsing

## Phase 3: Code Generation

- [ ] Generate Rust expressions
- [ ] Generate Rust functions
- [ ] Compile and test generated code

## Phase 4: Semantic Analysis

- [ ] Implement symbol tables
- [ ] Support variable declarations
- [ ] Validate types and scope

## Phase 5: CLI and Testing

- [ ] Implement CLI
- [ ] Add integration tests
- [ ] Improve error reporting

## Phase 6: Procedural Macros

- [ ] Create macro crate
- [ ] Integrate the transpiler
- [ ] Test macro expansion
