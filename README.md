# Python-to-Rust Transpiler

A learning-focused compiler engineering project
written in Rust.

## Overview

This project aims to translate a restricted,
statically typed Python-like language into
equivalent Rust source code.

The primary goal is to explore lexical analysis,
parsing, abstract syntax trees, semantic analysis,
and code generation.

## Project Scope

The initial implementation will support:

- Integer arithmetic
- Typed function declarations
- Variables and assignments
- Return statements
- Conditional statements
- While loops

The project does not aim to provide full Python
compatibility.

## Architecture

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

## Project Status

Under development.

Current phase: Project initialization.

## Documentation

- [Roadmap](docs/ROADMAP.md)
- [Technical Decisions](docs/DECISIONS.md)
