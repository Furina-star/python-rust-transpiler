# Project State

## Current Status

Phases 0, 1, and 1.5 are complete for the initial language scope. The library exports `lexer`, `token`, and `diagnostic`. **Phase 2: Syntax Analysis is next.** AST, parser, semantic analysis, and code generation files are empty placeholders and are not exported modules. There is no transpilation API, executable CLI, or macro crate yet.

## Completed Functionality

- Token recognition for the initial language subset.
- Indentation and dedentation, comments, blank lines, and parenthesized continuations.
- Positioned tokens and structured diagnostics, with string-error convenience APIs.
- Final `Newline`, block-closing `Dedent` tokens, and an explicit `Eof`.
- Unit tests for lexer behavior, token positions, and diagnostics, plus public API integration tests.

## Supported Lexer Behavior

| Feature | Current behavior |
| --- | --- |
| Integers | ASCII decimal digits parsed as `i32`; leading zeros accepted; signs are separate tokens |
| Identifiers | `[A-Za-z_][A-Za-z0-9_]*` |
| Booleans | Case-sensitive `True` and `False` |
| Keywords | Exact matches for `def`, `return`, `if`, `else`, `while`, `and`, `or`, `not` |
| Arithmetic | `+`, `-`, `*`, `/`, `//`, `%` |
| Assignment and comparisons | `=`, `==`, `!=`, `<`, `<=`, `>`, `>=` |
| Punctuation | `(`, `)`, `:`, `,`, `->` |
| Layout | Space-based `Indent`/`Dedent`; `Newline` between logical code lines and after the final statement |
| End of input | Remaining blocks close before `Eof`; input without code produces only `Eof` |
| Comments and continuations | Skip blank lines and `#` comments; suppress layout inside open parentheses |
| Line endings | Unix LF and Windows CRLF are accepted |
| Errors | Return a diagnostic with line and column for the first detected error; wrappers format it as a string |

Exact API types and layout token positions are documented in [Architecture](ARCHITECTURE.md).

## Known Limitations

- No parsing, grammar validation, type checking, execution, or Rust generation.
- Strings, floats, Unicode identifiers, and explicit backslash continuations are unsupported. Tabs and bare carriage returns in scanned code are rejected.
- Integer digit sequences must fit in `0..=2147483647`. Because signs are separate, `-2147483648` fails when its magnitude is scanned. Wider integer support is deferred.
- Letters or underscores immediately after digits are invalid integer literals. Numeric separators and non-decimal prefixes are unsupported.
- Compound assignment has no dedicated tokens or semantics: `+=` currently scans as `Plus`, `Equal`. It is deferred.
- Indentation increases are accepted lexically, even where no block is expected. Rejecting unexpected indentation is a planned parser responsibility.
- Positions record starts, not full spans or byte offsets. Error recovery and multiple diagnostics per call are not implemented.
- Type names such as `int` and names such as `divmod` remain ordinary identifiers without built-in semantics.

## Immediate Next Tasks

1. Define the AST and parser entry point over positioned tokens.
2. Parse expressions with precedence, then statements, blocks, and typed functions.
3. Add syntax diagnostics and tests, including unexpected indentation and explicit `Eof` handling.

See the [roadmap](ROADMAP.md) for later phases.

## Verification

Verified locally on 2026-10-23 with `rustc 1.98.1` and `cargo 1.98.1`:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | Passed |
| `cargo check --workspace --all-targets` | Passed, including compilation of the six standalone examples |
| `cargo clippy --workspace --lib --tests -- -D warnings` | Passed |
| `cargo test --workspace` | 82 unit tests and 2 integration tests passed; 0 failed |

The 82 unit tests comprise 80 lexer tests, one token test, and one diagnostic test. Cargo reports zero documentation tests. End-to-end transpilation tests await the remaining compiler stages. Examples were compiled, not run, during this verification.

The Rust snippets in the README and Architecture were also compiled and run separately with `rustdoc --test`; both passed. They are not included in Cargo's test count.

The local toolchain emitted a path canonicalization warning for `C:\Users\bello`; all checks completed successfully.

[Project overview](../README.md) · [Architecture](ARCHITECTURE.md) · [Technical Decisions](DECISIONS.md)
