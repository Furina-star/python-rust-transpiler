# Development Roadmap

Implementation milestones for the Python-to-Rust transpiler. Phase 1: Lexical Analysis is complete for the initial language scope. Syntax analysis is the next milestone; see [Project State](PROJECT_STATE.md) for supported behavior and verification results.

Checked items record completed milestones. The compiler phases below describe implementation order; the intended runtime pipeline places semantic analysis before code generation.

## Phase 0: Project Initialization

- [x] Configure Cargo workspace
- [x] Configure Git
- [x] Create initial documentation
- [x] Verify workspace builds

## Phase 1: Lexical Analysis

- [x] Define token types for the initial language scope
- [x] Tokenize integers, booleans, and ASCII identifiers
- [x] Recognize function, control flow, and logical keywords
- [x] Recognize arithmetic, assignment, and comparison operators
- [x] Recognize punctuation and return type arrows
- [x] Handle newlines, indentation, and dedentation
- [x] Ignore blank lines and comments
- [x] Support line continuations inside parentheses
- [x] Report invalid input with line numbers and column numbers for scanning errors
- [x] Validate lexer behavior with 65 unit tests
- [x] Verify public API tokenization and error reporting with two integration tests

## Phase 2: Syntax Analysis

- [ ] Define the AST
- [ ] Implement expression parsing
- [ ] Implement statement parsing
- [ ] Accept `Newline`, `Dedent`, and end-of-input as appropriate statement boundaries
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
- [ ] Add full end-to-end transpilation integration tests
- [ ] Improve error reporting

## Phase 6: Procedural Macros

- [ ] Create macro crate
- [ ] Integrate the transpiler
- [ ] Test macro expansion
