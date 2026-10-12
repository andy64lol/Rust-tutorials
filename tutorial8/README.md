# Tutorial 8 — Functions

Package reusable behaviour, and understand the expression/statement rule.

## What you'll learn

- Declaring functions with `fn`.
- Parameters (types required) and return types.
- Expressions vs statements and the semicolon rule.
- The `return` keyword and early exits.
- Recursion.
- Returning multiple values via tuples.

## Concepts

### Syntax

```
fn name(param: Type) -> ReturnType { body }
```

- Parameter types are **always required**.
- Omitting `->` means the return type is `()` (nothing).

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // trailing expression, no semicolon
}
```

### Expressions vs statements — the semicolon rule

- `a + b` is an **expression** with a value.
- `a + b;` is a **statement** producing `()`.

A function returns the value of its final expression, so **leave off the trailing
semicolon**. Forgetting this yields "expected `T`, found `()`".

### `return`

Used for early exits; at the end it is optional and usually omitted:

```rust
fn absolute(n: i32) -> i32 {
    if n < 0 { return -n; }
    n
}
```

### Recursion

A function that calls itself needs a **base case**:

```rust
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
```

### Multiple return values

Return a tuple and destructure it at the call site:

```rust
fn min_max(v: [i32; 5]) -> (i32, i32) { /* ... */ (min, max) }
let (lo, hi) = min_max(data);
```

### Function order

Top-level functions may be defined in any order; Rust does not require declarations
before use.

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial8.md`](../exercises/tutorial8.md).

## Previous / Next

[← Tutorial 7](../tutorial7/README.md) · [Tutorial 9 →](../tutorial9/README.md)
