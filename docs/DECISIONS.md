# Technical Decisions

## Decision 001: Implementation Language
Rust will be used for the transpiler.

## Decision 002: Initial Scope
The source language will be a restricted,
statically typed Python-like language.

Full Python compatibility is out of scope.

## Decision 003: Architecture
The transpiler will be developed as a
standalone library before introducing
CLI and procedural macro interfaces.

## Decision 004: Parsing Strategy
Begin with a hand-written lexer and
recursive-descent parser.

## Decision 005: Development Approach
Implement one component at a time.
Each component must be tested before
the next phase begins.

## Decision 006: Indentation and Line Continuations
Represent statement and block boundaries with
`Newline`, `Indent`, and `Dedent` tokens.
Use spaces for indentation and require dedentation
to match a previously recorded level.

Ignore blank lines and comments. Suppress layout
tokens within parentheses and close remaining
indentation levels at the end of input.

## Decision 007: Initial Lexical Scope
Use ASCII identifiers and `i32` integer literals.
Recognize booleans and keywords by exact,
case-sensitive matches after reading a full identifier.

Emit operators separately from their operands.
Type names remain identifiers for later compiler
stages to interpret and validate.

## Decision 008: Lexer Diagnostics
Keep the public lexer API as
`Result<Vec<Token>, String>` and stop at the first error.

Include line and column numbers for scanning errors
and line numbers for indentation and parenthesis errors.
Successful tokens do not yet include source spans.
