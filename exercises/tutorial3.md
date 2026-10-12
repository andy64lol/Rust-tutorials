# Exercises 03 — Primitive Data Types

**Goal:** use integers, floats, bools, chars, tuples, and arrays.

---

### 1. Declare everything
Declare one variable of each of these types and print them on separate lines:
`i8`, `u32`, `i64`, `f32`, `f64`, `bool`, `char`.

**Expected output (example, values are up to you)**
```
-12
4000000000
-9000000000
3.14
2.7182818
true
Z
```

---

### 2. Tuple practice
Create a tuple `(i32, bool, char)`. Print each element. Then:
1. Change the `i32` element to `99` and print it.
2. Replace the whole tuple with `(7, false, 'x')` and print all three elements.

---

### 3. Array practice
Create an array `[i32; 4]` with values `[5, 10, 15, 20]`.
1. Print the first and last elements.
2. Change the last element to `0` and print it.
3. Loop is **not** required — use indexes.

---

### 4. Which line fails?
These two lines are next to each other. One compiles, one does not. Say which and why:
```rust
let a: u8 = 200;
let b: u8 = 300;
```

---

### 5. Cast
Given `let small: u16 = 42;`, create a `u32` from it using `as` and print it.

**Expected output**
```
42
```

---

## Hints
- Unsigned types cannot hold negatives: `u8` is `0..=255`.
- A tuple's length and element types are fixed; you cannot add or remove fields.
- Arrays are `[type; length]` and all elements share one type.
