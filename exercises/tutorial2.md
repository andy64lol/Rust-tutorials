# Exercises 02 — Variables, Constants, and Shadowing

**Goal:** use `const`, `let`, `let mut`, scopes, and shadowing correctly.

---

### 1. Constants
Declare a `const` named `HOURS_IN_DAY: u32` equal to `24`, then print:
```
There are 24 hours in a day.
```

Then try to redeclare `HOURS_IN_DAY` with a different value **in the same scope**.
Observe the compile error and write a sentence explaining why constants cannot be
shadowed the way variables can.

---

### 2. Mutable counter
Create a mutable variable `counter` starting at `0`. Print it, then add `5`, print
again, then subtract `2`, print again.

**Expected output**
```
0
5
3
```

---

### 3. Shadowing with a type change
Create `let x = 10;`. Then shadow `x` with a string `"ten"`. Print both intermediate
results so the output is:
```
10
ten
```
Explain why this is impossible with `mut`.

---

### 4. Scopes
Inside `main`, print `outer`. Then open a block `{ ... }`, create a **new** variable
also called `name` and print `inner` from inside the block. After the block closes,
print `outer` again using the outer variable.

**Expected output**
```
outer
inner
outer
```

---

### 5. Scope + shadowing puzzle
Predict the output, then run it:
```rust
fn main() {
    let n = 1;
    {
        let n = n + 10;
        println!("{n}");
    }
    println!("{n}");
    let n = n * 100;
    println!("{n}");
}
```

---

## Hints
- `const` needs an explicit type and cannot be re-bound.
- A `{ }` block is a new scope; inner `let` bindings disappear when it ends.
- The right side of `let n = ...` is evaluated using the *old* `n`.
