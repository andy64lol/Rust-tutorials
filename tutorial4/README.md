# Tutorial 4 — Reading Console Input

How to read what the user types from standard input, and a first look at references
and error handling.

## What you'll learn

- `use std::io;` and modules.
- Creating a `String` with `String::new()`.
- `io::stdin().read_line(&mut input)` and why it needs `&mut`.
- Handling a `Result` with `.expect(...)`.

## Concepts

### Modules and `use`

The standard library is organised into modules. `use` brings a path into scope so you
can write short names:

```rust
use std::io;
// now `io::stdin()` instead of `std::io::stdin()`
```

### Creating the buffer

`read_line` appends into an existing `String`, so create one and make it mutable:

```rust
let mut input = String::new();
```

### Reading

```rust
io::stdin()
    .read_line(&mut input)
    .expect("failed to read line");
```

- `&mut input` is a **mutable borrow** — the method borrows your string to write into
  it, then hands ownership back. This is the first taste of Rust's borrowing.
- `read_line` reads up to and **including** the trailing newline, and returns a
  `Result<usize, io::Error>` (`Ok(n)` = bytes read).
- `.expect("...")` unwraps the `Result`, **panicking** if it was an `Err`.

> `String` is not an "object" in the OOP sense. It is a struct with methods (e.g.
> `.trim()`, `.len()`, `.push_str()`). Rust has no classes or inheritance.

### The trailing newline

Because the newline is kept, you almost always `.trim()` before using the value:

```rust
let cleaned = input.trim();
```

## Run it

```bash
cargo run
# then type something and press Enter
```

Non-interactive test:

```bash
echo "hello" | cargo run
```

## Practice

See [`../exercises/tutorial4.md`](../exercises/tutorial4.md).

## Previous / Next

[← Tutorial 3](../tutorial3/README.md) · [Tutorial 5 →](../tutorial5/README.md)
