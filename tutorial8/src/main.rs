//! # Tutorial 8 — Functions
//!
//! Functions let you name a reusable block of behaviour. You have already used the
//! special `main` function; here we define and call our own.
//!
//! ## Anatomy of a function
//!
//! ```text
//! fn name(param1: Type1, param2: Type2) -> ReturnType { body }
//! ```
//!
//! * `fn` declares a function.
//! * Parameter types are ALWAYS required; there is no inference across the call
//!   boundary, so both caller and callee agree on the contract.
//! * `-> ReturnType` gives the result type. If omitted, the return type is `()`.
//!
//! ## Expressions vs statements (the semicolon rule)
//!
//! This is the single most important detail in this tutorial:
//!
//! * `5 + 3` is an **expression**; it has the value `8`.
//! * `5 + 3;` (with a semicolon) is a **statement**; it runs and yields `()`.
//! * A function's return value is the value of its **final expression** — written
//!   WITHOUT a trailing semicolon. Writing `return x;` is also valid and equivalent,
//!   but idiomatic Rust usually omits `return` in favour of the trailing expression.
//!
//! If you accidentally add a trailing semicolon where you meant to return a value,
//! you will get "expected `T`, found `()`" — a very common beginner error.

fn main() {
    // Calling a function: name + parentheses with arguments.
    greet();

    // A function whose result we use. `add` returns an i32.
    let sum = add(3, 4);
    println!("3 + 4 = {}", sum);

    // Arguments are separated by commas; types are not written at the call site.
    println!("square of 6 = {}", square(6));

    // Functions can call other functions. `average` uses `add` internally.
    println!("average of 2,4,6,8,10 = {}", average(2, 4, 6, 8, 10));

    // `return` can be used for an early exit; here `absolute` also shows it.
    println!("absolute of -42 = {}", absolute(-42));

    // Recursion: a function that calls itself. Always needs a base case to stop.
    println!("5! = {}", factorial(5));
    println!("fib(10) = {}", fibonacci(10));

    // Functions can return MULTIPLE values by returning a tuple, which the caller
    // can destructure with a pattern.
    let (min, max) = min_max([4, 9, 1, 7, 3]);
    println!("min = {}, max = {}", min, max);
}

// --- FUNCTIONS THAT RETURN NOTHING (`()`) ---------------------------------------
//
// With no `->`, the return type is `()`, the "unit" type (an empty tuple that means
// "no meaningful value"). `greet` just performs a side effect (printing).
// Module-level functions can be declared in any order; Rust does not care whether
// `greet` appears before or after its use in `main`.
fn greet() {
    println!("Hello from greet()!");
}

// --- PARAMETERS AND RETURN VALUE -------------------------------------------------
//
// Two `i32` params, one `i32` return. The body is a single expression (`a + b`) with
// NO trailing semicolon, so its value is returned automatically.
fn add(a: i32, b: i32) -> i32 {
    a + b // trailing expression => returned
}

fn square(n: i32) -> i32 {
    // You could also write `return n * n;`. The one-liner below is preferred.
    n * n
}

// A function may take more parameters than this; here we accept five `i32`s.
// Note we deliberately keep the signature simple rather than using slices, which
// are introduced in tutorial 9.
fn average(a: i32, b: i32, c: i32, d: i32, e: i32) -> f64 {
    // `add` returns i32, but we want a fractional average, so cast to f64 BEFORE
    // dividing by 5.0. Then call `add` a few times to sum everything.
    let total = add(add(add(add(a, b), c), d), e);
    total as f64 / 5.0
}

// --- THE `return` KEYWORD ---------------------------------------------------------
//
// `return` sends a value back immediately, ending the function. It is required for
// an early exit (the trailing-expression style can only return at the very end).
// The final `-n` here is a trailing expression, reached only when `n < 0` is false.
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n; // early return for negatives
    }
    n // otherwise return n as the trailing expression
}

// --- RECURSION --------------------------------------------------------------------
//
// `factorial(n) = n * factorial(n - 1)`, with base case `factorial(0) = 1`.
// Without the base case this would recurse forever and overflow the stack.
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Naive Fibonacci: `fib(0) = 0`, `fib(1) = 1`, `fib(n) = fib(n-1) + fib(n-2)`.
// This recomputes values exponentially; fine for learning, slow for large `n`.
fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// --- RETURNING A TUPLE (multiple values) ------------------------------------------
//
// The return type is `(i32, i32)` — a tuple. The caller can bind both parts with
// `let (min, max) = ...;`. `usize` parameters/return here are just to demonstrate a
// different integer type.
fn min_max(values: [i32; 5]) -> (i32, i32) {
    // Start both with the first element, then compare the rest.
    let mut min = values[0];
    let mut max = values[0];
    for value in values.iter() {
        if *value < min {
            min = *value; // `*value` dereferences the `&i32` the iterator yields
        }
        if *value > max {
            max = *value;
        }
    }
    (min, max) // return the pair as a trailing tuple expression
}
