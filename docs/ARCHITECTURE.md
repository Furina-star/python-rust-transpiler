# Architecture

## Pipeline and Status

```text
Source -> Lexer -> Parser -> AST -> Semantic Analysis -> Rust Code Generator -> Rust Source
```

Phases 0, 1, and 1.5 are complete. The lexer and its supporting types are implemented; all later stages are planned. The target is a restricted, statically typed Python-like language, not full Python compatibility.

## Modules and Responsibilities

The workspace contains one dependency-free library crate, `crates/transpiler`. Paths below are relative to its `src/` directory.

| File | Responsibility | Status |
| --- | --- | --- |
| `lib.rs` | Public module exports | Exports `lexer`, `token`, and `diagnostic` |
| `token.rs` | `Token`, `SourcePosition`, and `LocatedToken` | Implemented |
| `diagnostic.rs` | Structured error data and display formatting | Implemented |
| `lexer.rs` | Scanning, layout, continuations, and lexical diagnostics | Implemented |
| `lexer/tests.rs` | Lexer unit tests | Loaded by the lexer test module |
| `ast.rs` | Expression, statement, and function representations | Empty placeholder; not exported |
| `parser.rs` | Recursive-descent parsing into the AST | Empty placeholder; not exported |
| `semantic.rs` | Name, type, and scope validation | Empty placeholder; not exported |
| `codegen.rs` | Rust source generation | Empty placeholder; not exported |

Token and diagnostic unit tests are in their respective modules. Public API integration tests are in `crates/transpiler/tests/`. The root `cli/` and `python_macro/` directories contain only `.gitkeep` files and are not workspace members.

## Tokens and Positions

- `Token` represents integer and boolean literals, identifiers, keywords, operators, punctuation, and layout/end markers. It derives `Debug`, `Clone`, `PartialEq`, and `Eq`.
- `SourcePosition { line: usize, column: usize }` uses one-based lines and character columns. Columns count Rust `char` values, not bytes or display width. It is also `Copy`.
- `LocatedToken { token: Token, position: SourcePosition }` pairs a token with its start position. It derives `Debug`, `Clone`, `PartialEq`, and `Eq`.

Positions do not include token lengths or end spans. See [Supported Lexer Behavior](PROJECT_STATE.md#supported-lexer-behavior) for the token vocabulary and numeric limits.

## Lexer APIs

All three public functions are in `lexer` and accept `source: &str`.

| Function | Return type | Purpose |
| --- | --- | --- |
| `tokenize_with_diagnostics` | `Result<Vec<LocatedToken>, Diagnostic>` | Core API with structured errors |
| `tokenize_with_positions` | `Result<Vec<LocatedToken>, String>` | Positioned tokens with formatted errors |
| `tokenize` | `Result<Vec<Token>, String>` | Token values with formatted errors |

The wrappers delegate to the same core lexer. `tokenize_with_positions` formats diagnostics; `tokenize` additionally removes positions. Existing signatures remain available, but callers must account for the final `Newline` and `Eof` tokens.

The private `tokenize_line_located` scanner uses lookahead to recognize operators, reads complete names before matching keywords, and parses decimal digits as `i32`. The core lexer manages indentation and opening-parenthesis stacks across lines.

## Layout and End of Input

- Space-only blank lines and comment-only lines are ignored. Tabs on scanned lines are rejected.
- Outside parentheses, a `Newline` separates consecutive code lines. Its position is one character past the end of the previous physical code line, including any trailing spaces or comment text.
- Increased indentation emits `Indent`. A decrease must match an existing level and emits one `Dedent` per closed block. These tokens use the current line and column `leading_spaces + 1`.
- While parentheses remain open, subsequent lines emit no layout tokens. Opening positions are retained for diagnostics.
- At the end of code, emit `Newline`, any remaining `Dedent` tokens, then `Eof`. All use one character past the last physical code line. A trailing LF or CRLF does not add another newline token.
- Input containing no code emits only `Eof` at line 1, column 1.

For example, `x=1` ends with `Integer(1), Newline, Eof`; an open block ending in `y=1` ends with `Integer(1), Newline, Dedent, Eof`.

The lexer checks indentation levels, not whether a statement permits a block. The planned parser will reject unexpected indentation and consume statement and block boundaries according to the grammar.

## Diagnostics

`Diagnostic { position: SourcePosition, message: String }` derives `Debug`, `Clone`, `PartialEq`, and `Eq`. Its `Display` implementation produces `Line N, Column M: message`.

The lexer returns the first detected error without recovery. Private `ScanError` values gain a line number when converted to diagnostics. Indentation and parenthesis errors are constructed directly: invalid dedents point to the first code column, unmatched closing parentheses to the closing token, and unclosed parentheses to the most recent unmatched opening token.

```rust
use python_rust_transpiler::{
    lexer::tokenize_with_diagnostics,
    token::SourcePosition,
};

let error = tokenize_with_diagnostics("x=@").unwrap_err();
assert_eq!(error.position, SourcePosition { line: 1, column: 3 });
assert_eq!(error.message, "Unexpected character: @");
assert_eq!(error.to_string(), "Line 1, Column 3: Unexpected character: @");
```

## Next Stage

Phase 2 will define the AST and parse expressions, statements, and typed functions using positioned tokens. `Newline`, `Indent`, `Dedent`, and `Eof` provide explicit boundaries. Semantic analysis and Rust generation will follow; the intended runtime pipeline validates semantics before code generation.

[Project overview](../README.md) · [Project State](PROJECT_STATE.md) · [Roadmap](ROADMAP.md) · [Technical Decisions](DECISIONS.md)
