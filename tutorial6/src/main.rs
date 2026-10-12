//! # Tutorial 6 — Conditionals and Boolean Logic
//!
//! This tutorial introduces branching: making the program choose between paths.
//!
//! * **Comparison operators** — `==`, `!=`, `<`, `>`, `<=`, `>=`.
//! * **Logical operators** — `&&` (and), `||` (or), `!` (not).
//! * **`if` / `else if` / `else`**.
//! * The crucial Rust rule: **a condition must be a `bool`**. There is no "truthy"
//!   coercion like in JavaScript, Python, or C. A number is NOT a valid condition.
//! * `if` is an **expression**, so it can produce a value that you assign.
//!
//! ## Expression vs statement
//!
//! An *expression* evaluates to a value (`2 + 3`, `if ... { 1 } else { 2 }`).
//! A *statement* performs an action and produces no value. In Rust most things,
//! including `if`, are expressions — this is why `let x = if ... else ...;` works.

// We will read a number from the user at the end, so bring in `io`.
use std::io;

fn main() {
    // --- COMPARISON OPERATORS ---------------------------------------------------
    //
    // Comparisons produce a `bool` (`true`/`false`). They do NOT produce numbers
    // like they do in C (where `1`/`0` are often used). This keeps types honest.
    let a = 10;
    let b = 20;

    println!("a == b: {}", a == b); // equality
    println!("a != b: {}", a != b); // inequality
    println!("a <  b: {}", a < b);  // less than
    println!("a >  b: {}", a > b);  // greater than
    println!("a <= b: {}", a <= b); // less than or equal
    println!("a >= b: {}", a >= b); // greater than or equal

    // --- LOGICAL OPERATORS ------------------------------------------------------
    //
    // `&&` (AND) is true only if BOTH sides are true.
    // `||` (OR)  is true if AT LEAST ONE side is true.
    // `!`  (NOT) flips a bool.
    // Both `&&` and `||` are short-circuiting: the right side is only evaluated
    // if the left side does not already decide the outcome. This matters when the
    // right side has side effects or could be expensive/unsafe.
    let is_adult = true;
    let has_ticket = false;

    println!("adult AND ticket: {}", is_adult && has_ticket); // false
    println!("adult OR  ticket: {}", is_adult || has_ticket); // true
    println!("NOT adult:        {}", !is_adult); // false

    // --- IF / ELSE IF / ELSE ----------------------------------------------------
    //
    // The condition MUST be a `bool`. `if a { ... }` where `a` is an integer is a
    // compile error ("expected `bool`, found integer").
    if a < b {
        println!("a is smaller than b");
    } else if a == b {
        println!("a equals b");
    } else {
        println!("a is bigger than b");
    }

    // Braces are REQUIRED around every branch — `if cond statement;` is not allowed,
    // unlike in C. This avoids the classic "goto fail" brace bug.

    // --- IF IS AN EXPRESSION ----------------------------------------------------
    //
    // Because `if` is an expression, you can use it on the right-hand side of `let`.
    // Every branch must produce a value of the SAME type. Here, both branches yield
    // an `i32`, so `label` is `i32`.
    let n = 7;
    let label = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, label);

    // You can also use a block's value directly. Note there is NO semicolon after
    // `n + 1` — a trailing semicolon would turn the expression into a statement
    // (producing `()`), which would not match the other branch's `i32`.
    let magnitude = if n < 0 { -n } else { n };
    println!("magnitude of {} is {}", n, magnitude);

    // --- A SMALL PROGRAM: FIZZBUZZ-LIKE CHECK -----------------------------------
    //
    // Read a number and classify it using the operators above.
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let number: i64 = input.trim().parse().unwrap();

    if number % 15 == 0 {
        // divisible by 3 AND 5 -> divisible by 15
        println!("fizzbuzz");
    } else if number % 3 == 0 {
        println!("fizz");
    } else if number % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", number);
    }
}
