# Rust Tutorials

A hands-on, progressive tour of the Rust programming language. Each tutorial is a
self-contained Cargo binary whose `src/main.rs` is heavily commented: it explains not
just *what* the code does, but *why* the language works that way. Every tutorial folder
also has its own `README.md` that explains the concepts, and a matching practice set in
[`exercises/`](exercises/README.md).

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

| #  | Tutorial | Docs | Topic |
| -- | -------- | ---- | ----- |
| 1  | `tutorial1` | [README](tutorial1/README.md) | Basic output (`println!` / `print!`, macros, `main`) |
| 2  | `tutorial2` | [README](tutorial2/README.md) | Variables, constants, mutability (`mut`), scopes, and shadowing |
| 3  | `tutorial3` | [README](tutorial3/README.md) | Primitive data types: integers, floats, bools, chars, tuples, arrays |
| 4  | `tutorial4` | [README](tutorial4/README.md) | Reading console input from `stdin`, `String`, and `Result` |
| 5  | `tutorial5` | [README](tutorial5/README.md) | Arithmetic, integer division, overflow, and type casting (`as`) |
| 6  | `tutorial6` | [README](tutorial6/README.md) | Conditionals (`if`/`else`) and boolean logic (`&&`, `\|\|`, `!`) |
| 7  | `tutorial7` | [README](tutorial7/README.md) | Loops: `loop`, `while`, `for`, `break`, `continue`, and labels |
| 8  | `tutorial8` | [README](tutorial8/README.md) | Functions: parameters, return values, expressions vs statements |
| 9  | `tutorial9` | [README](tutorial9/README.md) | Ownership, moves, borrowing, references, `String` vs `&str`, slices |
| 10 | `tutorial10`| [README](tutorial10/README.md) | Structs, methods, enums, and pattern matching with `match` |

## Exercises

Each tutorial has a matching practice set in [`exercises/`](exercises/README.md)
(`tutorial1.md` … `tutorial10.md`). Work through them after reading the tutorial's
source. They include expected outputs and hints (but no checked-in solutions).

## Interactive website

There is a Duolingo-style interactive version of these lessons under [`site/`](site/README.md):
a path of units, XP, streaks, hearts, and per-exercise feedback. It is a
dependency-free static site; lesson content is data-driven from
`site/data/lessons.json`, and your progress can be exported/imported as JSON.

```bash
cd site
python3 -m http.server 8000
# open http://localhost:8000
```

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
├── exercises/
│   ├── README.md          # exercise index
│   ├── tutorial1.md
│   ├── ...
│   └── tutorial10.md
├── site/                  # Duolingo-style interactive lessons
│   ├── README.md
│   ├── index.html
│   ├── styles.css
│   ├── app.js
│   └── data/lessons.json  # all lesson content
├── tutorial1/
│   ├── README.md      # concept explanation
│   ├── Cargo.toml
│   └── src/main.rs    # heavily documented source
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
