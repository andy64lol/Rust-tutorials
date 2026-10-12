//! # Tutorial 7 — Loops and Control Flow
//!
//! Rust has three looping constructs:
//!
//! * `loop { ... }` — repeats forever until you explicitly `break`.
//! * `while cond { ... }` — repeats while a condition is true.
//! * `for item in iterator { ... }` — repeats once per element of a sequence.
//!   This is the most idiomatic and usually the one you want.
//!
//! Two keywords steer control flow inside any loop:
//!
//! * `break` — leave the loop immediately (and optionally yield a value from `loop`).
//! * `continue` — skip the rest of the current iteration and start the next one.
//!
//! ## Why `for` is preferred
//!
//! A `for` loop over a collection cannot accidentally run off the end or forget to
//! increment an index, so Rust encourages it over manual index counting. Use `while`
//! when the number of iterations is genuinely unknown, and `loop` for retry/state
//! machines where you break from the middle.

fn main() {
    // --- `loop`: INFINITE LOOP WITH A VALUE -------------------------------------
    //
    // `loop` runs forever unless broken. Because the compiler knows a `loop` only
    // exits via `break`, a `break value` can make the whole `loop` expression
    // evaluate to `value`. `while` and `for` cannot do this (they always yield `()`).
    let mut counter = 0;
    let result = loop {
        counter += 1; // `+=` is shorthand for `counter = counter + 1`
        if counter == 5 {
            break counter * 10; // exit and make `result` equal 50
        }
    };
    println!("loop produced {}", result); // 50

    // --- `while`: CONDITION-CONTROLLED LOOP -------------------------------------
    //
    // Checks the condition before each iteration. If it is false the first time,
    // the body never runs.
    let mut countdown = 3;
    while countdown > 0 {
        println!("countdown {}", countdown);
        countdown -= 1;
    }
    println!("liftoff!");

    // --- `for` + RANGES ---------------------------------------------------------
    //
    // Ranges are values too:
    //   `a..b`  — from a UP TO but NOT including b (half-open)
    //   `a..=b` — from a THROUGH b (inclusive)
    for i in 1..5 {
        print!("{} ", i); // 1 2 3 4
    }
    println!();

    for i in 1..=5 {
        print!("{} ", i); // 1 2 3 4 5
    }
    println!();

    // A step that goes backwards is fine as long as the bounds make sense.
    for i in (1..=3).rev() {
        print!("{} ", i); // 3 2 1
    }
    println!();

    // --- `continue` and `break` -------------------------------------------------
    //
    // Print only the odd numbers from 1 to 10. `continue` skips the print for even
    // numbers; `break` would stop early (not used here).
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // skip the remainder of this iteration
        }
        print!("{} ", i); // 1 3 5 7 9
    }
    println!();

    // --- `for` OVER AN ARRAY ----------------------------------------------------
    //
    // Iterating an array yields its elements directly; no index bookkeeping needed.
    // `iter()` hands out references (`&i32`), but `for` auto-dereferences them here,
    // so `value` can be used as a plain `i32`.
    let numbers = [10, 20, 30, 40];
    let mut total = 0;
    for value in numbers.iter() {
        total += value;
    }
    println!("sum = {}", total); // 100

    // If you DO need the index, use `.enumerate()`, which yields `(index, value)`.
    for (index, value) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", index, value);
    }

    // --- NESTED LOOPS AND LABELS ------------------------------------------------
    //
    // A bare `break` only exits the INNERMOST loop. To break out of an outer loop,
    // put a label (an identifier starting with a single quote) on it and `break 'label`.
    'outer: for row in 1..=3 {
        for col in 1..=3 {
            if row == 2 && col == 2 {
                println!("found the target at ({row}, {col}), stopping everything");
                break 'outer; // leaves BOTH loops
            }
            print!("({row},{col}) ");
        }
    }
    println!();

    // --- A CLASSIC EXAMPLE: FIZZBUZZ 1..=15 -------------------------------------
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}
