# Exercises 06 — Conditionals and Boolean Logic

**Goal:** use `if`/`else if`/`else`, comparisons, and `&&` / `||` / `!`.

---

### 1. Even or odd
Read an integer and print `even` or `odd`.

---

### 2. Largest of three
Read three integers and print the largest one. (Do not use a loop or arrays.)

**Example (inputs 4, 9, 7)**
```
largest = 9
```

---

### 3. Leap year
A year is a leap year if it is divisible by 4, **except** years divisible by 100 but
not by 400. Implement the rule with one boolean expression and print `leap` or
`not leap` for a year read from stdin.

Test cases (input → output):
- `2024` → `leap`
- `1900` → `not leap`
- `2000` → `leap`

---

### 4. Grade letter
Read a score `0..=100` and print the grade using these bands:
`>=90 A`, `>=80 B`, `>=70 C`, `>=60 D`, otherwise `F`. Handle scores above 100 as
`invalid`.

---

### 5. `if` as an expression
Write `let parity = if n % 2 == 0 { "even" } else { "odd" };` and print it. Then try
adding a semicolon after `"odd"` and explain the compile error.

---

### 6. Predict
What does this print?
```rust
let x = 5;
let y = if x > 3 && x < 10 { 1 } else { 0 };
println!("{y}");
```

---

## Hints
- Divisibility check: `n % k == 0`.
- Combine conditions with parentheses for clarity, e.g.
  `(y % 4 == 0) && (y % 100 != 0 || y % 400 == 0)`.
- A condition must be a `bool`; `if n { }` for an integer will not compile.
