# Tutorial 5 — Arithmetic and Type Casting

Doing maths in Rust, where the type rules are strict and explicit.

## What you'll learn

- Rust never converts numeric types implicitly.
- Integer division truncates.
- Overflow behaviour in debug vs release.
- Literal suffixes and underscores.
- Parsing numbers from input.

## Concepts

### No implicit conversions

`i16 + i32` will not compile; both operands must share a type. Convert explicitly:

```rust
let x: u8 = 9;
let y: i8 = 10;
let z = x as i8 + y; // cast one side
```

`as` binds tighter than arithmetic operators, so `x as i8 + y` means `(x as i8) + y`.

### Integer division truncates

```rust
let z = 9 / 10; // 0, not 0.9
```

Cast to a float **before** dividing to keep the fraction:

```rust
let z = x as f32 / y as f32;
```

### Remainder (`%`)

```rust
let r = 255 % 10; // 5
```

### Overflow

Doing `255u8 + 1` compiles but overflows at runtime:

- **Debug builds** (`cargo run`): it **panics** ("attempt to add with overflow").
- **Release builds** (`cargo run --release`): it **wraps** around (two's complement).

Explicit alternatives: `.wrapping_add`, `.checked_add` (returns `Option`),
`.saturating_add` (clamps), `.overflowing_add` (wrap + flag).

### Literal suffixes & separators

```rust
let a = 32.0f32;     // suffix
let b = 32_i16;      // suffix after underscore
let c = 127_000_i64; // underscores are visual only -> 127000
```

### Parsing input

```rust
let n: i64 = input.trim().parse().unwrap();
```

`.parse()` converts text to the type demanded by the annotation and returns a `Result`,
which `.unwrap()` turns into a value or a panic.

### Float precision

Casting `f64 -> f32` can lose precision because `f32` has fewer significand bits.

## Run it

```bash
cargo run
# then type a number, e.g. 40, and press Enter
```

## Practice

See [`../exercises/tutorial5.md`](../exercises/tutorial5.md).

## Previous / Next

[← Tutorial 4](../tutorial4/README.md) · [Tutorial 6 →](../tutorial6/README.md)
