# Tutorial 9 — Ownership, Borrowing, and References

The core idea that lets Rust guarantee memory safety without a garbage collector.

## What you'll learn

- The three ownership rules.
- Move vs copy.
- Shared (`&T`) and exclusive (`&mut T`) references.
- The borrowing rule (many readers XOR one writer).
- `String` vs `&str`.
- Slices (`&[T]`, string slices).

## Concepts

### The three rules of ownership

1. Every value has exactly one owner.
2. There is only one owner at a time.
3. When the owner goes out of scope, the value is **dropped** (freed).

### Move vs copy

- `Copy` types (all scalars, and tuples/arrays of them) are **copied** on assignment;
  the original stays usable.

```rust
let a = 5;
let b = a; // copy
println!("{a} {b}"); // both OK
```

- Heap-owning types like `String` are **moved**; the original becomes invalid:

```rust
let s = String::from("hi");
let t = s; // moved
// println!("{s}"); // ERROR: use of moved value
```

Use `.clone()` for an explicit deep copy when you really need two owners.

### Borrowing

Lend a value instead of moving it:

- `&T` — shared (read-only) reference.
- `&mut T` — exclusive (read/write) reference; the source must be `mut`.

```rust
fn length(text: &String) -> usize { text.len() }
fn append(text: &mut String) { text.push_str(", world"); }
```

### The borrow rule

At any given time you may have **either** many shared borrows **or** exactly one
mutable borrow — never both. This is checked at compile time and prevents data races.

### `String` vs `&str`

- `String` — owned, growable, heap-allocated UTF-8 buffer.
- `&str` — borrowed view into UTF-8 text owned elsewhere (a `String` or a literal).

Functions that only read text should take `&str`; it accepts both `&String` (via deref
coercion) and `&'static str` literals.

```rust
fn first_word(text: &str) -> &str { /* ... */ }
```

### Slices

A slice is a borrowed view of part of a collection, with no copying:

```rust
let data = [3, 1, 4, 1, 5, 9];
let middle = &data[1..4]; // [1, 4, 1]
fn sum(values: &[i32]) -> i32 { /* ... */ }
```

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial9.md`](../exercises/tutorial9.md).

## Previous / Next

[← Tutorial 8](../tutorial8/README.md) · [Tutorial 10 →](../tutorial10/README.md)
