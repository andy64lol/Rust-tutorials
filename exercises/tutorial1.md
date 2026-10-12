# Exercises 01 — Basic Output

**Goal:** get comfortable with `fn main`, `println!`, and `print!`.

---

### 1. Hello, you
Print `Hello, world!` on its own line.

**Expected output**
```
Hello, world!
```

---

### 2. About me
Print three lines about yourself (name, age, hobby) using **one** `println!` call and
escape sequences (`\n`).

**Expected output (example)**
```
Name: Ada
Age: 36
Hobby: programming
```

---

### 3. Same line
Using `print!` (no newline) and then a single `println!` at the end, produce:
```
Hello World!
```

---

### 4. Predict the output
Read this snippet and write down what it prints **before** running it. Then check:
```rust
fn main() {
    print!("a");
    print!("b");
    println!("c");
    println!("d");
}
```

---

## Hints
- `println!` adds a newline after the text; `print!` does not.
- To make a newline *inside* a string, use `\n`.
- The last `println!` decides where the final newline lands.
