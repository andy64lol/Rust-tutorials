# Exercises

Practice problems for each tutorial. Do them **in order** — each set assumes only the
concepts taught up to and including its tutorial.

## How to work through these

1. Create a scratch project (or reuse the tutorial's folder) to type your code:
   ```bash
   cargo new --bin scratch
   cd scratch
   ```
2. Write a solution, run it with `cargo run`, and compare against the **Expected
   output** noted in each exercise.
3. Only look at the **Hints** after you have tried on your own. Struggling and reading
   the compiler errors is the point.

## Sets

| Set | Tutorial | Topic |
| --- | -------- | ----- |
| [01](tutorial1.md)  | Basic output | `println!`, `print!` |
| [02](tutorial2.md)  | Variables | `const`, `let`, `mut`, shadowing, scopes |
| [03](tutorial3.md)  | Data types | integers, floats, bools, chars, tuples, arrays |
| [04](tutorial4.md)  | Console input | `stdin`, `String`, `.trim()` |
| [05](tutorial5.md)  | Arithmetic | casts, integer division, overflow, `parse` |
| [06](tutorial6.md)  | Conditionals | `if`/`else`, boolean logic |
| [07](tutorial7.md)  | Loops | `loop`, `while`, `for`, `break`, `continue` |
| [08](tutorial8.md)  | Functions | params, returns, recursion |
| [09](tutorial9.md)  | Ownership | moves, borrowing, `&str`, slices |
| [10](tutorial10.md) | Structs & enums | methods, `match`, pattern matching |

> There are no official solutions checked in. If you get stuck, re-read the matching
> tutorial's `src/main.rs` — its comments are the reference.
