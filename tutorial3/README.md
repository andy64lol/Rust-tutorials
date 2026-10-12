# Tutorial 3 — Primitive Data Types

Rust's built-in scalar and compound types, and how to read, write, and index them.

## What you'll learn

- Integers: signed (`iN`) and unsigned (`uN`), and their ranges.
- Floating point: `f32` and `f64`.
- Booleans and characters.
- Tuples and arrays.
- Type casting with `as`.
- The `Copy` behaviour of scalar types.

## Concepts

### Integers

| Signed (can be negative) | Unsigned (zero and up) |
| ------------------------ | ---------------------- |
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` |

The number is the bit width. Both signed and unsigned of the same width hold the same
count of distinct values; the signed one spends half on negatives:

```
i8 -> -128..=127      u8 -> 0..=255      (256 values each)
```

- Integer literals default to **`i32`**.
- `isize`/`usize` are pointer-sized and used for indexing/sizes.

### Floating point

- `f32` — single precision.
- `f64` — double precision; the **default** for float literals.

```rust
let a: f32 = 3.14;
let b = 2.718281828; // f64
```

### Bools and chars

```rust
let t: bool = true;
let c: char = 'a';   // single quotes, one Unicode scalar value (4 bytes)
```

A `char` is not a `&str`; `'x'` is a character, `"x"` is a string slice.

### Tuples

A fixed-length group of values that may have **different** types. Access by `.index`,
starting at `0`:

```rust
let mut tup: (i32, bool, char) = (1, true, 's');
println!("{}", tup.0);
tup.0 = 10; // mutable element assignment
```

Tuples do not implement `Display`, so `{}` cannot print a whole tuple.

### Arrays

A fixed-length group of values of the **same** type. The type is `[T; N]` and both the
element type and the length are part of it. Index with `[i]`, starting at `0`; out-of-
bounds indexing panics at runtime:

```rust
let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
println!("{}", arr[0]);
arr[4] = 7;
```

### Casting with `as`

Explicit numeric conversion. Integer→integer truncates to the target width; float→int
truncates toward zero:

```rust
let v: i128 = x as i128;
```

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial3.md`](../exercises/tutorial3.md).

## Previous / Next

[← Tutorial 2](../tutorial2/README.md) · [Tutorial 4 →](../tutorial4/README.md)
