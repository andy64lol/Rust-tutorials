# Rust Tutorials

A hands-on, progressive tour of the Rust programming language. Each tutorial is a
self-contained Cargo binary whose `src/main.rs` is heavily commented: it explains not
just *what* the code does, but *why* the language works that way.

The tutorials build on each other, so reading them in order is recommended.

## Requirements

- [Rust toolchain](https://rustup.rs/) (installed via `rustup`). The projects use
  `edition = "2024"`, so a recent stable toolchain is required.
  Run `rustc --version` to check.

## Running a tutorial

Each folder is an independent Cargo project. Move into the one you want and run it:

```bash
cd tutorial1
cargo run
```

`cargo run` compiles the project (if needed) and executes it. Some tutorials read from
standard input; type your value and press Enter when prompted.

To run without the terminal (e.g. for a scripted check):

```bash
echo 42 | cargo run
```

## Curriculum

| #  | Folder      | Topic                                                              |
| -- | ----------- | ------------------------------------------------------------------ |
| 1  | `tutorial1` | Basic output (`println!` / `print!`, macros, `main`)               |
| 2  | `tutorial2` | Variables, constants, mutability (`mut`), scopes, and shadowing    |
| 3  | `tutorial3` | Primitive data types: integers, floats, bools, chars, tuples, arrays |
| 4  | `tutorial4` | Reading console input from `stdin`, `String`, and `Result`         |
| 5  | `tutorial5` | Arithmetic, integer division, overflow, and type casting (`as`)    |
| 6  | `tutorial6` | Conditionals (`if`/`else`) and boolean logic (`&&`, `\|\|`, `!`)   |
| 7  | `tutorial7` | Loops: `loop`, `while`, `for`, `break`, `continue`, and labels     |
| 8  | `tutorial8` | Functions: parameters, return values, expressions vs statements    |
| 9  | `tutorial9` | Ownership, moves, borrowing, references, `String` vs `&str`, slices |
| 10 | `tutorial10`| Structs, methods, enums, and pattern matching with `match`         |

## Conventions used throughout

- Older tutorials (1–5) were originally written with inline `//` comments; these have
  been expanded with crate-level `//!` docs and detailed explanations.
- Newer tutorials (6–10) follow the same style: a `//!` header describing the topic,
  then sectioned comments (`// --- TITLE ---`) explaining each concept.
- Code is written to be **correct and idiomatic**, but favouring clarity over cleverness.

## Project layout

```
rust/
├── README.md
├── .gitignore
├── tutorial1/
│   ├── Cargo.toml
│   └── src/main.rs   # heavily documented source
├── tutorial2/
├── ...
└── tutorial10/
```

Each tutorial produces a fresh `target/` directory and a `Cargo.lock` when built;
both are ignored by git (see `.gitignore`).

## Tips for learners

- Read the source in an editor with rust-analyzer for inline type hints.
- Break things on purpose: delete a semicolon, change a type, try to mutate an
  immutable binding. The compiler errors are the best teacher in Rust.
- `cargo clippy` and `cargo fmt` are your friends once you are comfortable.
