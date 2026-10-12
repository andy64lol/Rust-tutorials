# Tutorial 1 — Basic Output

The smallest useful Rust program. It shows the entry point every binary needs and the
two macros used to print text to the terminal.

## What you'll learn

- The `main` function as the program's entry point.
- `println!` vs `print!`.
- What a macro is and why it ends with `!`.
- Escape sequences such as `\n`.

## Concepts

### `fn main`

Execution always begins in `main`. For a program that produces no meaningful result it
takes no arguments and returns nothing (the "unit" type, `()`):

```rust
fn main() {
    // ...
}
```

### Macro calls end with `!`

`println!` and `print!` are **macros**, not ordinary functions — note the `!`. Macros
are expanded at compile time, which lets them accept a variable number of arguments and
validate the format string while compiling.

### `println!` vs `print!`

- `println!` prints the text **and a newline** (`\n`).
- `print!` prints exactly what you give it, **without** a newline.

```rust
println!("line one"); // ends with a newline
print!("no newline here");
println!("...continues on the same line");
```

### Escape sequences

Inside a string you can embed special characters with a backslash:

| Sequence | Meaning        |
| -------- | -------------- |
| `\n`     | newline        |
| `\t`     | tab            |
| `\"`     | a double quote |
| `\\`     | a backslash    |

## Run it

```bash
cargo run
```

**Expected output**

```
Hello, world!
hii
```

## Practice

See [`../exercises/tutorial1.md`](../exercises/tutorial1.md).

## Next

[Tutorial 2 — Variables](../tutorial2/README.md)
