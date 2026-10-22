# Project State

## Implementation Status

Phase 1: Lexical Analysis is complete for the initial language scope. The library exposes `lexer::tokenize` and `token::Token`. Parsing, AST definitions, semantic analysis, and code generation remain placeholders; a complete transpilation API and CLI are not yet available.

## Completed

- Cargo workspace and library structure.
- Compiler token types and public lexer API for the initial language scope.
- Integer, boolean, identifier, keyword, operator, and punctuation tokenization.
- Newline, indentation, and dedentation handling, including block closure at the end of input.
- Blank line and comment handling, with line continuations inside parentheses.
- Error locations for invalid input, integer overflow, indentation, and unbalanced parentheses.
- 65 lexer unit tests in `crates/transpiler/src/lexer/tests.rs`, including a complete typed function.

## Next Milestones

- Define AST types for expressions, statements, and functions.
- Implement recursive-descent expression parsing with operator precedence.
- Implement statement and typed function parsing, including block boundaries.
- Add parser tests for valid syntax and syntax errors.

See the [roadmap](ROADMAP.md) for subsequent compiler phases.

## Supported Lexer Behavior

The public API is `tokenize(source: &str) -> Result<Vec<Token>, String>`.

| Feature | Behavior |
| --- | --- |
| Integers | Groups ASCII digits into `i32` values |
| Booleans | Recognizes the case-sensitive literals `True` and `False` |
| Arithmetic operators | Recognizes `+`, `-`, `*`, `/`, `//`, and `%`; signs remain separate from integer tokens |
| Assignment and comparisons | Recognizes `=`, `==`, `!=`, `<`, `<=`, `>`, and `>=` |
| Identifiers | Accepts ASCII letters or `_` initially, followed by ASCII letters, digits, or `_` |
| Keywords | Matches `def`, `return`, `if`, `else`, `while`, `and`, `or`, and `not` as complete, case-sensitive identifiers |
| Punctuation | Recognizes parentheses, colons, commas, and the return type arrow `->` |
| Spaces and empty input | Skips ordinary spaces; accepts empty input |
| Newlines | Accepts Unix and Windows line endings; emits `Newline` between code lines outside parentheses, with no extra token for a trailing line ending |
| Indentation | Uses leading spaces to emit `Indent` and `Dedent`; dedentation must match an existing level, and remaining blocks close at the end of input |
| Comments and blank lines | Ignores inline comments, comment-only lines, and blank lines |
| Line continuations | Suppresses newline and indentation tokens inside open parentheses |
| Scanning errors | Reports the first unsupported character, integer overflow, tab in code, or bare carriage return with a one-based line and character column |
| Structural errors | Reports inconsistent indentation or unmatched closing parentheses on the offending line, and unclosed parentheses on the opening line |

## Limitations

- The language subset excludes strings, floating-point literals, Unicode identifiers, and additional Python syntax.
- Tabs in code and explicit backslash line continuations are unsupported; indentation uses spaces.
- Integer digit sequences must fit in `i32`. A minus sign is a separate token, so `-2147483648` currently fails when its magnitude is scanned.
- Tokens do not include source spans. Errors are strings, and structural errors currently report a line without a column.
- Tokenization does not validate grammar, types, or scope. Type names and names such as `divmod` are identifiers, without built-in semantics.
- Automated compiler coverage consists of lexer unit tests; integration and documentation tests are pending.

## Verification

Run from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
```

Verified on 2026-10-23 with `rustc 1.98.1` and `cargo 1.98.1`:

| Check | Result |
| --- | --- |
| Formatting | Passed |
| Workspace check, including all targets | Passed |
| Unit tests | 65 passed |
| Documentation tests | None defined |
| Standalone examples | All six ran successfully |

Toolchain commands emitted a local path canonicalization warning; verification completed successfully.

## Documentation Maintenance

Update supported behavior, limitations, and verification results when the implementation changes. Keep the [roadmap](ROADMAP.md) aligned with completed milestones and record architecture or scope changes in [Technical Decisions](DECISIONS.md).
