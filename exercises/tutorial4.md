# Exercises 04 — Console Input

**Goal:** read user input with `std::io` and `String`.

> All of these read from standard input. When testing non-interactively, pipe input in:
> `echo "Ada" | cargo run`.

---

### 1. Greet the user
Ask for a name, read a line, and print `Hello, <name>!`.

Tip: `read_line` keeps the trailing newline, so use `.trim()` before printing.

**Interactive**
```
What is your name?
Ada
Hello, Ada!
```

---

### 2. Two lines
Read two separate lines (a first name and a last name) and print them joined:
```
First name: Ada
Last name: Lovelace
Your name is Ada Lovelace
```

---

### 3. Echo with length
Read a line and print it back **and** its length in characters.
```
Enter text: hello
You typed "hello" (5 characters)
```
> Use `.trim().len()` for now. (Byte vs character length is a later topic.)

---

### 4. Predict: what is printed?
If the user types `hi` and presses Enter, what does `.len()` report for the raw
`input` **before** `.trim()`? Why? (Try it.)

---

## Hints
- `let mut input = String::new();`
- `io::stdin().read_line(&mut input).expect("...");`
- `use std::io;` must be at the top of the file.
