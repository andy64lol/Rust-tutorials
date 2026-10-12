# Exercises 09 — Ownership and Borrowing

**Goal:** reason about moves, borrows, `String` vs `&str`, and slices.

---

### 1. Move or copy?
For each snippet, say whether the last line compiles, and why.
```rust
// (a)
let a = 10;
let b = a;
println!("{a} {b}");
```
```rust
// (b)
let s = String::from("hi");
let t = s;
println!("{s}");
```
```rust
// (c)
let s = String::from("hi");
let t = s.clone();
println!("{s} {t}");
```

---

### 2. Fix the borrow
The following does not compile. Change as little as possible to make it compile while
keeping `s` usable afterwards:
```rust
fn main() {
    let s = String::from("hello");
    print_it(s);
    println!("{s}");
}
fn print_it(text: String) {
    println!("{text}");
}
```

---

### 3. Append without consuming
Write `fn shout(text: &mut String)` that uppercases… actually just appends `"!!!"`.
Call it on a `String` and print the result.

**Expected output**
```
Go!!!
```

---

### 4. First word
Write `fn first_word(text: &str) -> &str` that returns the part before the first space.
Test with a literal and an owned `String` (pass `&owned`).

**Example**
```
first of "sea of stars" = sea
```

---

### 5. Slice sum and slice max
Given `let data = [4, 8, 15, 16, 23, 42];`
1. Write `fn sum(values: &[i32]) -> i32`.
2. Write `fn max(values: &[i32]) -> i32`.
3. Print the sum and max of the whole array **and** of the slice `&data[1..4]`.

**Expected output**
```
total = 108
max = 42
slice total = 39
slice max = 16
```

---

### 6. Explain the borrow error
Why does this fail to compile, and what is the smallest fix?
```rust
let mut v = String::from("count");
let r = &v;
v.push_str("!");
println!("{r}");
```

---

## Hints
- Primitive types are `Copy`; `String` is not.
- Borrow to avoid moving: `&T` to read, `&mut T` to modify (variable must be `mut`).
- You may have many shared borrows **or** one mutable borrow, never both at once.
- `&String` coerces to `&str`; a string literal is already `&str`.
