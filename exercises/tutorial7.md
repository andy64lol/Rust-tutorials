# Exercises 07 — Loops and Control Flow

**Goal:** use `loop`, `while`, `for`, `break`, `continue`, ranges, and labels.

---

### 1. Count to ten
Print `1` through `10`, each on its own line.

---

### 2. Sum of 1..=100
Compute the sum of the integers from 1 to 100 **without** a formula, using a loop.
Print the sum.

**Expected output**
```
5050
```

---

### 3. Multiplication table
Read an integer `n` and print its multiplication table from 1 to 10, formatted as:
```
3 x 1 = 3
3 x 2 = 6
...
3 x 10 = 30
```

---

### 4. Skip multiples
Print all numbers from 1 to 30 that are **not** divisible by 3, using `continue`.

---

### 5. Find and stop
Using a `while` loop, find the smallest integer `k` such that `k * k > 500`, then
print `k` and stop. (Loop with `break`.)

**Expected output (k should be 23)**
```
first square above 500 is 23^2 = 529
```

---

### 6. Nested loops with a label
Using labeled loops, search pairs `(i, j)` with `1 <= i, j <= 5` for the first pair
where `i * j == 12`, print it, and break out of **both** loops.

---

### 7. FizzBuzz
Print 1..=30 where multiples of 3 → `Fizz`, of 5 → `Buzz`, of both → `FizzBuzz`, else
the number.

---

### 8. Reverse iteration
Print `5 4 3 2 1` on one line using a reversed range.

---

## Hints
- Half-open `1..n` excludes `n`; inclusive `1..=n` includes it.
- `.rev()` reverses an iterator; `.enumerate()` adds an index.
- Label a loop `'outer:` and use `break 'outer;` to leave nested loops.
