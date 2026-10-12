# Tutorial 2 — Variables, Constants, and Shadowing

How Rust stores values in names, and the difference between constants, immutable
variables, mutable variables, and shadowing.

## What you'll learn

- `const` for compile-time constants.
- `let` bindings are immutable by default.
- `let mut` for values you intend to change.
- Scopes created by `{ ... }` blocks.
- Shadowing with a new `let`, including changing the type.

## Concepts

### Constants — `const`

Declared at compile time, must be typed, always immutable, and conventionally written
in `SCREAMING_SNAKE_CASE`:

```rust
const SECONDS_IN_MINUTES: u32 = 60;
```

A `const` is inlined at each use; it cannot be re-bound or re-typed later.

### Immutable variables — `let`

Bindings are **immutable by default**. Assigning to one again is a compile error:

```rust
let x = 4;
// x = 5; // ERROR: cannot assign twice to immutable variable
```

### Mutable variables — `let mut`

Add `mut` when you need to change the value. The **type stays the same**:

```rust
let mut x = 4;
x = 5;      // OK
// x = "hi"; // ERROR: expected integer, found &str
```

### Scopes

A `{ ... }` block introduces a new scope. Bindings created inside it are gone when the
block ends:

```rust
let x = 5;
{
    let x = x - 2; // a NEW x, only inside this block
    println!("{x}"); // 3
}
println!("{x}"); // 5 — outer x untouched
```

### Shadowing

Re-declaring with `let` creates a new binding that hides the old one. Unlike `mut`,
shadowing **can change the type**:

```rust
let x = 6;
let x = x + 1;      // 7 (reads the old x)
let x = "Hello";    // now a &str
```

### Summary table

| Feature      | `const`   | `let`     | `let mut` | shadowing (`let` again) |
| ------------ | --------- | --------- | --------- | ----------------------- |
| Mutable?     | no        | no        | yes       | new binding each time   |
| Type change? | no        | no        | no        | yes                     |
| Typed always?| yes       | inferred* | inferred* | inferred*               |

\* inference is usually fine for `let`; `const` requires an explicit type.

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial2.md`](../exercises/tutorial2.md).

## Previous / Next

[← Tutorial 1](../tutorial1/README.md) · [Tutorial 3 →](../tutorial3/README.md)
