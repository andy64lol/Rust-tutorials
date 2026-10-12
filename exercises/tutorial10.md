# Exercises 10 — Structs, Methods, Enums, and `match`

**Goal:** model data with structs/enums and act on it with methods and `match`.

---

### 1. A `Student` struct
Define
```rust
struct Student {
    name: String,
    grade: u32, // 0..=100
}
```
Add:
- an associated function `Student::new(name: &str, grade: u32) -> Student`,
- a method `is_passing(&self) -> bool` (grade `>= 60`),
- a method `letter(&self) -> char` returning `A/B/C/D/F` by the bands from Tutorial 6.

Create two students and print their name, passing status, and letter.

---

### 2. `Point` methods
Define a tuple struct `Point(i32, i32)` with:
- `fn new(x, y) -> Point`,
- `fn translate(&mut self, dx, dy)`,
- `fn distance_squared(&self, other: &Point) -> i32` (no square root needed).

Translate a point and print the distance squared to the origin.

---

### 3. `Direction` enum
Define
```rust
enum Direction { North, South, East, West }
```
Write `fn delta(dir: &Direction) -> (i32, i32)` returning the unit step for each
direction, using `match`. Print the delta for `East`.

**Expected output**
```
East -> (1, 0)
```

---

### 4. Shape areas with `match`
Reuse the idea from Tutorial 10: an enum `Shape` with `Circle(f64)`,
`Rect(u32, u32)`, and `Square(u32)`. Implement `area(&self) -> f64` using `match`.
Create one of each and print the areas rounded to 2 decimals.

---

### 5. `Option`-style handling
Given `let maybe: Option<i32> = Some(7);` (from the standard library):
- Print the value plus 1 using `match`.
- Do the same with `if let`.
- Then set it to `None` and print a default of `-1` when absent.

---

### 6. Pattern-match numbers
Write a `match` on an integer that prints:
- `"zero"` for `0`,
- `"small"` for `1..=9`,
- `"medium"` for `10..=99`,
- `"large"` otherwise.

Test with `0, 5, 42, 1000`.

---

### 7. Method that consumes
Add a method `into_rectangle(self) -> Rectangle`-style transform to one of your
structs. Explain (in a comment) why calling it means the original value can no longer
be used.

---

## Hints
- `#[derive(Debug)]` lets you print with `{:?}`.
- `&self` reads, `&mut self` modifies, `self` consumes.
- In `match`, list every variant; use `_` only as a deliberate catch-all.
- `if let Some(v) = x { ... }` handles a single variant conveniently.
