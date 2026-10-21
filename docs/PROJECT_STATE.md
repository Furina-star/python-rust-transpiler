# Project State

## Implementation Status

Lexical analysis is in progress. The library exposes `lexer::tokenize` and `token::Token`. Parsing, AST definitions, semantic analysis, and code generation remain placeholders; a complete transpilation API and CLI are not yet available.

## Completed

- Cargo workspace and library structure.
- Initial compiler token types and public lexer API.
- Integer, addition/subtraction, identifier, and keyword tokenization.
- Error handling for integer overflow and unsupported characters.
- Lexer unit tests covering supported tokens, input boundaries, and errors.

## Next Milestones

- Recognize the punctuation variants already defined in `Token`.
- Implement newline and indentation handling.
- Extend lexer coverage before beginning syntax analysis.

See the [roadmap](ROADMAP.md) for subsequent compiler phases.

## Supported Lexer Behavior

The public API is `tokenize(source: &str) -> Result<Vec<Token>, String>`.

| Feature | Behavior |
| --- | --- |
| Integers | Groups ASCII digits into `i32` values |
| Operators | Emits separate `Plus` and `Minus` tokens |
| Identifiers | Accepts ASCII letters or `_` initially, followed by ASCII letters, digits, or `_` |
| Keywords | Matches the complete, case-sensitive identifiers `def` and `return` |
| Spaces and empty input | Skips ordinary spaces; accepts empty input |
| Errors | Returns a string error for integer overflow or the first unsupported character |

## Limitations

- Parentheses, colons, and commas have token variants but are not yet recognized.
- Tabs, newlines, indentation, other operators, and additional keywords are unsupported.
- Tokens and errors do not include source locations.
- Automated coverage consists of lexer unit tests; integration and documentation tests are pending.

## Verification

Run from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
```

Verified on 2026-10-21 with `rustc 1.98.1` and `cargo 1.98.1`:

| Check | Result |
| --- | --- |
| Formatting | Passed |
| Workspace check, including all targets | Passed |
| Unit tests | 13 passed |
| Documentation tests | None defined |
| Standalone examples | All six ran successfully |

Toolchain commands emitted a local path canonicalization warning; verification completed successfully.

## Documentation Maintenance

Update supported behavior, limitations, and verification results when the implementation changes. Keep the [roadmap](ROADMAP.md) aligned with completed milestones and record architecture or scope changes in [Technical Decisions](DECISIONS.md).
