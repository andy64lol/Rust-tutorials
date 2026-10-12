# Exercises 08 — Functions

**Goal:** define functions with parameters and return values; use recursion.

---

### 1. `is_even`
Write `fn is_even(n: i32) -> bool` and use it to print whether 4 and 7 are even.

**Expected output**
```
4 even: true
7 even: false
```

---

### 2. `max_of_three`
Write `fn max_of_three(a: i32, b: i32, c: i32) -> i32` and print the maximum of
`(3, 9, 5)`.

---

### 3. `power`
Write `fn power(base: i64, exp: u32) -> i64` that computes `base^exp` with a loop
(no `pow`). Test `power(2, 10)`.

**Expected output**
```
1024
```

---

### 4. `count_vowels`
Write `fn count_vowels(text: &str) -> usize` that counts `a, e, i, o, u`
(case-insensitive). Test with `"Hello World"`.

**Expected output**
```
3
```

---

### 5. `is_prime`
Write `fn is_prime(n: u32) -> bool`. Test `1`, `2`, `9`, and `97`.

**Expected output**
```
1 prime: false
2 prime: true
9 prime: false
97 prime: true
```

---

### 6. `fibonacci` vs loop
Write `fn fib_recursive(n: u32) -> u64` and `fn fib_iterative(n: u32) -> u64`, and
check they agree for `n = 20`. Reflect: which one is faster and why?

---

### 7. Multiple returns
Write `fn min_max(values: [i32; 5]) -> (i32, i32)` returning `(min, max)`, and print
the pair for `[8, -3, 12, 0, 5]`.

---

## Hints
- A function returns its final expression — leave off the trailing `;`.
- `return x;` works for early exits but is optional at the end.
- Recurrences need a base case to terminate.
