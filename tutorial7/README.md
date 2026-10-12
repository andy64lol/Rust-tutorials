# Tutorial 7 — Loops and Control Flow

Repeat work with `loop`, `while`, and `for`.

## What you'll learn

- `loop { }` — repeat until `break`.
- `while cond { }` — repeat while a condition holds.
- `for x in iter { }` — iterate a sequence (preferred).
- `break` and `continue`.
- Ranges, `.rev()`, `.enumerate()`.
- Loop labels for breaking out of nested loops.

## Concepts

### `loop`

Runs forever until `break`. A `break value` makes the whole `loop` evaluate to that
value (something `while`/`for` cannot do):

```rust
let result = loop {
    counter += 1;
    if counter == 5 {
        break counter * 10;
    }
};
```

### `while`

Checks the condition before each iteration; may run zero times:

```rust
while countdown > 0 {
    countdown -= 1;
}
```

### `for` and ranges

```rust
for i in 1..5 { }   // 1 2 3 4   (half-open: excludes 5)
for i in 1..=5 { }  // 1 2 3 4 5 (inclusive)
```

Prefer `for` over manual index counting — it cannot run past the end or forget to
increment.

### `break` and `continue`

- `break` leaves the loop immediately.
- `continue` skips the rest of this iteration.

```rust
for i in 1..=10 {
    if i % 2 == 0 { continue; }
    print!("{i} "); // odd numbers only
}
```

### Iterating collections

```rust
for value in numbers.iter() { /* &i32 */ }
for (i, value) in numbers.iter().enumerate() { /* index + value */ }
```

### Nested loops and labels

A bare `break` only exits the innermost loop. Label an outer loop and break it by name:

```rust
'outer: for row in 1..=3 {
    for col in 1..=3 {
        if row == 2 && col == 2 { break 'outer; }
    }
}
```

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial7.md`](../exercises/tutorial7.md).

## Previous / Next

[← Tutorial 6](../tutorial6/README.md) · [Tutorial 8 →](../tutorial8/README.md)
