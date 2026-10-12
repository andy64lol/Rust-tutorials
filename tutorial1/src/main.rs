//! # Tutorial 1 — Basic Output
//!
//! This is the smallest possible useful Rust program. It demonstrates two things:
//!
//! 1. The required `main` function, which is where every Rust binary starts executing.
//! 2. The two most common macros for writing text to **standard output** (`stdout`):
//!    `println!` (prints a line, adds a newline) and `print!` (prints raw text, no newline).
//!
//! ## Why are they "macros" and not functions?
//!
//! A macro call is marked with a trailing `!` (`println!`). Macros are expanded at
//! **compile time** into ordinary Rust code. This matters because `println!` can do
//! things a normal function cannot:
//!
//! * It accepts a **format string** (`"Hello"`, `"x = {}"`, ...) that is validated at
//!   compile time, so a mismatched number of `{}` placeholders becomes a compile error.
//! * It takes a variable number of arguments.
//! * It can capture variables by reference in modern Rust ("implicit named arguments"
//!   like `{x}`), as you will see in later tutorials.
//!
//! ## Where does the output go?
//!
//! Both macros write to `stdout` and lock that stream for the duration of the call.
//! `println!` appends a `\n` (newline) at the end; `print!` does not. That is why the
//! final `print!("hii")` below continues on the same line as the previous `println!`.

// The `main` function:
//   * `fn`    — keyword that declares a function.
//   * `main`  — the special name the Rust runtime looks for as the program entry point.
//   * `()`    — an empty parameter list (this function takes no arguments).
//   * No `->` — the return type is implicitly `()` (the "unit" type, i.e. nothing),
//               because the body does not end in an expression that returns a value.
fn main() {
    // `println!` prints the given text followed by a newline (`\n`).
    // `print!` (used below) prints the text with NO trailing newline.
    //
    // The `{}` "double curly brace" is a placeholder used when you have values to
    // interpolate; this call has none, so the literal string is printed as-is.
    println!("Hello, world!"); // prints "Hello, world!" and moves to a new line

    // Because `print!` does not add a newline, "hii" is appended right after the
    // newline produced by the previous `println!`, and the cursor stays on that line.
    print!("hii");
}
