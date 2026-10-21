# Development Roadmap

Implementation milestones for the Python-to-Rust transpiler. Lexical analysis is currently in progress; see [Project State](PROJECT_STATE.md) for supported behavior and verification results.

Checked items record completed milestones, not a guarantee that every repository check passes. The compiler phases below describe implementation order; the intended runtime pipeline places semantic analysis before code generation.

## Phase 0: Project Initialization

- [x] Configure Cargo workspace
- [x] Configure Git
- [x] Create initial documentation
- [x] Verify workspace builds

## Phase 1: Lexical Analysis

- [x] Define initial token types
- [x] Tokenize integers and addition/subtraction operators
- [x] Recognize identifiers and `def`/`return` keywords
- [x] Validate initial lexer behavior with unit tests
- [ ] Recognize punctuation
- [ ] Handle newlines and indentation
- [ ] Extend tests for remaining lexer features

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
