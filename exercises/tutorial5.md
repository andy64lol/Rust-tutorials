# Exercises 05 — Arithmetic and Type Casting

**Goal:** master `as`, integer division, overflow, and `.parse()`.

---

### 1. Celsius to Fahrenheit
Given `let celsius: f64 = 25.0;`, compute `fahrenheit = celsius * 9.0 / 5.0 + 32.0`
and print it.

**Expected output**
```
25°C = 77°F
```

---

### 2. Integer division trap
Given `let a: i32 = 7; let b: i32 = 2;`
1. Print `a / b`. Explain why it is not `3.5`.
2. Compute and print the true value `3.5` by casting both operands to `f64` **before**
   dividing.

**Expected output**
```
3
3.5
```

---

### 3. Reading two numbers
Read two integers from stdin and print their sum, difference, product, and quotient
(integer division). Example for inputs `10` and `3`:
```
sum = 13
difference = 7
product = 30
quotient = 3
```

---

### 4. Overflow: predict and explain
For each, say whether it **compiles**, and what happens at runtime in a debug build:
```rust
let a: u8 = 255;
let b = a + 1;
```
```rust
let a: i8 = 127;
let b = a + 1;
```
Then rewrite each using `wrapping_add` to make it well-defined.

---

### 5. Literal styles
Write the number one million as: (a) a plain literal, (b) with underscores, (c) with a
type suffix for `i64`. Print all three.

**Expected output**
```
1000000
1000000
1000000
```

---

## Hints
- `as` converts numeric types explicitly; Rust never converts for you.
- Integer `/` truncates toward zero.
- Debug builds panic on overflow; `wrapping_add`/`checked_add`/`saturating_add` are
  explicit alternatives.
- `input.trim().parse::<i64>().unwrap()` parses text to a number.
