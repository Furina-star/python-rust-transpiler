# Project State

## Current Position

- **Foundation:** Phase 0 (Cargo workspace and repository setup) completed.
- **Learning:** Rust fundamentals completed through Day 5 and the integration review, with six runnable practice examples.
- **Compiler implementation:** Not started; Phase 1 (Lexical Analysis) is next. The library entry point and compiler source files are placeholders; no transpilation API or CLI is available.

## Completed

- Configured the Cargo workspace with `crates/transpiler` as its only member.
- Day 1: Variables, mutability, types, functions, and return expressions. The current [basics example](../crates/transpiler/examples/rust_basics.rs) calculates damage and healing using a `Champion` struct.
- Day 2: Defined and instantiated a struct, updated fields, declared an enum, and implemented exhaustive pattern matching in [the structs example](../crates/transpiler/examples/rust_structs.rs).
- Implemented associated data with `PlayerStatus::Respawning(u32)` and bound the countdown in `PlayerStatus::Respawning(seconds)`.
- Day 3: Created and grew a `Vec` with `push`, accessed elements by index, and checked its length in [the vectors example](../crates/transpiler/examples/rust_vectors.rs).
- Practiced owned `String` values, `push_str`, and borrowed `&str` text.
- Traversed and summed vector values with `while` and `for` loops; used `iter()` for shared access, `iter_mut()` to update elements, and `into_iter()` to consume a vector.
- Day 4: Used `Vec::get` and matched `Some` / `None` for present and missing elements in [the options example](../crates/transpiler/examples/rust_options.rs).
- Matched `Ok` / `Err` for integer parsing and propagated `ParseIntError` with `?` in `parse_damage`.
- Converted parse errors to `String` with `map_err` and `?` in `validate_damage`; accepted positive and zero damage, rejected negative damage with a custom error, and handled invalid input.
- Day 5: Practiced moving an owned `String`, copying integers, block scope, and cloning into an independently mutable string in [the ownership example](../crates/transpiler/examples/rust_ownership.rs).
- Borrowed strings through `&str`, updated an owned string through `&mut String`, returned an owned label from `make_label`, and returned a borrowed view tied to the input lifetime through `view_label`.
- Integration review: Combined `Token::Integer(i32)` and `Token::Plus`, `Vec<Token>`, integer parsing, `map_err`, and `?` in [the integration example](../crates/transpiler/examples/rust_integration.rs).
- Constructed three tokens from two integer strings, matched their variants through a shared borrow, and reused the vector afterward to print its length.
- Built and ran all six practice examples successfully.

## Current Task

Begin Phase 1: Lexical Analysis. Define compiler token types in `src/token.rs`, implement arithmetic tokenization in `src/lexer.rs`, and add lexer tests. Keep practice examples separate from the compiler source files.

## Fundamentals Review

The ownership example demonstrates that shared and mutable borrows let the original owner keep using its string. `view_label` returns a reference to its input; Rust infers the matching lifetime through lifetime elision.

The integration example correctly parses both inputs before returning a token vector. `map_err` converts each parse error to `String`, and `?` returns early if parsing fails. `display_tokens` borrows the vector, so `main` can still call `tokens.len()` afterward.

- **Verified input:** `build_expression("50", "75")` produces `Integer(50)`, `Plus`, and `Integer(75)`; the displayed count is `3`.
- **Scope:** The example constructs an addition expression from two separate inputs. It does not scan source text, preserve original lexemes, or evaluate addition; compiler lexical analysis remains unimplemented.
- **Small improvement:** `display_tokens` could accept `&[Token]` instead of `&Vec<Token>` because it only reads elements. The current signature works correctly for this example.
- **Coverage follow-up:** The integration example's `main` exercises only successful parsing. Invalid, empty, and out-of-range inputs should be covered when adding automated parsing or lexer tests.

## Known Issues / Follow-ups

- The previous formatting follow-up is resolved: `cargo fmt --all -- --check` now passes.
- The examples compile with `dead_code` warnings: `Champion.ability_power`, `Champion.defense`, and `Player.level` are unread, and `PlayerStatus::Alive` and `PlayerStatus::Dead` are never constructed.
- Toolchain commands emit an environment warning about being unable to canonicalize `C:\Users\bello`; checks and example runs still succeed.
- There are no automated tests yet. Passing the current test harness does not establish compiler correctness.
- Per-day learning notes are not currently present in the working tree. Add links only after the corresponding files exist.

## Verification Commands

Run these commands from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo run -p python-rust-transpiler --example rust_basics
cargo run -p python-rust-transpiler --example rust_structs
cargo run -p python-rust-transpiler --example rust_vectors
cargo run -p python-rust-transpiler --example rust_options
cargo run -p python-rust-transpiler --example rust_ownership
cargo run -p python-rust-transpiler --example rust_integration
```

## Verification Results

Reviewed on 2026-10-21 using `rustc 1.98.1` and `cargo 1.98.1`:

| Check | Result |
| --- | --- |
| Workspace check, including all targets | Passed with the unused-code warnings listed above |
| Workspace tests | Passed; 0 unit tests and 0 documentation tests |
| Formatting check | Passed |
| `rust_basics` example | Passed; printed `Ahri's remaining health: 1000` |
| `rust_structs` example | Passed; output shown below |
| `rust_vectors` example | Passed; printed `Faker T1`, `Ahri`, total damage `1000`, updated values `[150, 250, 350, 450]`, total damage `1200`, and owned values `1`, `2`, `3` |
| `rust_options` example | Passed; found damage `200`, handled a missing element, parsed `123`, and handled invalid parsing; validation returned `Ok(250)`, `Ok(0)`, `Err("Damage cannot be negative")` for `-25`, and `Err("invalid digit found in string")` for `invalid` |
| `rust_ownership` example | Passed; printed moved and copied values, kept `Furina` unchanged while extending its clone, updated `Neuvillette` to `Neuvillette the Iudex`, and displayed a borrowed label while retaining the original name |
| `rust_integration` example | Passed; printed `Integer: 50`, `Plus`, `Integer: 75`, and `Token count: 3` |

```text
Faker initial health: 100
Faker current health: 80
Faker experiences: 50
Faker is respawning in 5 seconds...
```

## Documentation Convention

At the end of each study day:

1. Review the implementation and concepts learned.
2. Run relevant checks and examples; record failures and warnings as well as successes.
3. Update this file and the [roadmap](ROADMAP.md) to reflect verified progress.
4. If keeping per-day notes, create `docs/learning/` and the relevant `DAY_XX.md` file before linking to it.
5. Update `DECISIONS.md` only when an actual architecture or scope decision changes.
6. Commit verified code and documentation together when ready.

## Last Review

2026-10-21: Reviewed ownership, borrowing, and the integration exercise; recorded completion of Rust fundamentals and reran the checks and all six examples listed above. Formatting passes; existing unused-code and environment warnings remain. Phase 1 (Lexical Analysis) is next.
