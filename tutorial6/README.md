# Tutorial 6 — Conditionals and Boolean Logic

Make the program choose between paths.

## What you'll learn

- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- Logical operators: `&&`, `||`, `!`.
- `if` / `else if` / `else`.
- Conditions must be `bool` (no "truthiness").
- `if` is an expression that yields a value.

## Concepts

### Comparisons produce `bool`

```rust
10 == 20 // false
10 <  20 // true
```

### Logical operators

- `&&` — AND (both true).
- `||` — OR (at least one true).
- `!`  — NOT (flip).

Both `&&` and `||` **short-circuit**: the right side runs only if the left doesn't
already decide the result.

### `if` / `else if` / `else`

```rust
if a < b {
    println!("smaller");
} else if a == b {
    println!("equal");
} else {
    println!("bigger");
}
```

Braces are **required** around every branch — no `if cond statement;`.

### Conditions must be `bool`

There is no truthy/falsy conversion. `if 1 { }` is a compile error; write `if 1 != 0`.
This avoids a whole class of bugs common in C-like languages.

### `if` as an expression

Every branch must produce a value of the **same type**, and the value is the last
expression in the block (no trailing `;`):

```rust
let label = if n % 2 == 0 { "even" } else { "odd" };
```

## Run it

```bash
cargo run
# then enter a number, e.g. 9
```

## Practice

See [`../exercises/tutorial6.md`](../exercises/tutorial6.md).

## Previous / Next

[← Tutorial 5](../tutorial5/README.md) · [Tutorial 7 →](../tutorial7/README.md)
