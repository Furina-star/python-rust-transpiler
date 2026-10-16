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
    