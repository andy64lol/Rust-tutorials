# Tutorial 10 — Structs, Methods, Enums, and `match`

Model your own data and act on it — Rust's alternative to class hierarchies.

## What you'll learn

- Defining structs with named fields.
- Tuple structs.
- `impl` blocks: methods (`&self`, `&mut self`, `self`) and associated functions.
- Enums, including variants that carry data.
- Exhaustive pattern matching with `match`, plus `if let`.

## Concepts

### Structs

Group related fields into one named type. `#[derive(...)]` auto-generates trait impls
like `Debug` (print with `{:?}`), `Clone`, and `PartialEq`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Tuple structs

Named type with positional fields:

```rust
struct Point(i32, i32);
let p = Point(1, 2);
println!("{}", p.0);
```

### `impl` blocks

Attach behaviour to a type. Three kinds of receivers:

| Receiver     | Meaning                                         |
| ------------ | ----------------------------------------------- |
| `&self`      | borrow to read; value stays usable              |
| `&mut self`  | borrow to modify                                |
| `self`       | consume (move) the value — can only be called once |

An **associated function** has no `self` and is called as `Type::name(...)`, typically
a constructor:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle { Rectangle { width, height } }
    fn area(&self) -> u32 { self.width * self.height }
    fn scale(&mut self, factor: u32) { self.width *= factor; self.height *= factor; }
}
```

`Rectangle { width, height }` uses **field-init shorthand** (`width` means
`width: width`).

### Enums

A value that is exactly one of several variants, each possibly holding data:

```rust
enum Shape {
    Circle(f64),
    Rect(u32, u32),
    Triangle { base: f64, height: f64 },
}
```

Enums + `match` replace class hierarchies and make illegal states unrepresentable.

### `match`

Pattern matching is **exhaustive**: you must handle every variant, so adding a variant
forces you to update every `match`. Arms can bind data and can include guards (`if`):

```rust
let kind = match shape {
    Shape::Circle(_) => "round",
    Shape::Rect(_, _) => "boxy",
    Shape::Triangle { .. } => "pointy",
};
```

`if let` is a shortcut when you care about only one variant:

```rust
if let Some(v) = maybe { println!("{v}"); }
```

### `Option<T>`

The standard library enum for "a value or nothing": `Some(T)` / `None`. It replaces
null and forces you to handle the missing case.

## Run it

```bash
cargo run
```

## Practice

See [`../exercises/tutorial10.md`](../exercises/tutorial10.md).

## Previous

[← Tutorial 9](../tutorial9/README.md)

---

You've reached the end of the tutorial series. From here, good next topics are modules
and crates, traits and generics, `Vec`/`HashMap`, error handling with `Result` and `?`,
lifetimes, and concurrency.
