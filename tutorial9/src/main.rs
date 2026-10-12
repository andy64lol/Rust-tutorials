//! # Tutorial 9 — Ownership, Borrowing, and References
//!
//! Ownership is the feature that lets Rust guarantee memory safety WITHOUT a garbage
//! collector. Understanding it is the biggest hurdle when learning Rust, so we build
//! it up carefully.
//!
//! ## The three ownership rules
//!
//! 1. Every value has exactly ONE owner (a variable).
//! 2. There can be only one owner at a time.
//! 3. When the owner goes out of scope, the value is **dropped** (its memory is freed).
//!
//! ## Move vs copy
//!
//! * Types that can be copied bit-for-bit cheaply (all the "scalar" types and tuples/
//!   arrays of them) implement `Copy`. Assigning them COPIES.
//! * Heap-owning types like `String` do NOT implement `Copy`. Assigning them **MOVES**
//!   ownership: the original variable becomes unusable. This prevents a "double free"
//!   bug where two variables would try to free the same memory.
//!
//! ## Borrowing
//!
//! Rather than move a value into a function just to read it, you can lend it out with
//! a **reference** (`&T` for shared/read-only, `&mut T` for exclusive/write). Borrowing
//! rules enforced by the compiler:
//!
//! * At any moment you may have EITHER many `&T` (shared) references
//!   OR exactly one `&mut T` (exclusive) reference — never both at once.
//! * References must never outlive the data they point to (no dangling pointers).

fn main() {
    // --- COPY: SIMPLE STACK TYPES -------------------------------------------------
    //
    // `i32` is `Copy`, so `b = a` copies the bits and BOTH remain valid.
    let a = 5;
    let b = a; // copy
    println!("a = {}, b = {}", a, b); // both usable

    // --- MOVE: HEAP-OWNING `String` ----------------------------------------------
    //
    // `String` owns heap memory, so it cannot be `Copy`. `s2 = s1` MOVES ownership to
    // `s2`; `s1` is invalidated. Uncommenting the print of `s1` is a compile error:
    // "borrow of moved value: `s1`".
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2
    // println!("{}", s1); // ERROR: s1 was moved
    println!("s2 = {}", s2);

    // If you truly need two independent copies on the heap, call `.clone()`. This is
    // an explicit, potentially expensive deep copy — Rust makes you ask for it.
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // --- MOVING INTO A FUNCTION ---------------------------------------------------
    //
    // Passing `s3` to `takes_ownership` MOVES it. After the call, `s3` is gone.
    takes_ownership(s3);
    // println!("{}", s3); // ERROR: s3 was moved into the function

    // But `i32` is `Copy`, so `a` is still available after this call.
    copies_only(a);
    println!("a is still usable: {}", a);

    // --- BORROWING: SHARED REFERENCES (`&T`) -------------------------------------
    //
    // To let a function read a value without taking ownership, pass a reference.
    // `&s2` creates a shared borrow. `s2` remains owned by `main` and is still usable.
    let len = calculate_length(&s2);
    println!("the length of '{}' is {}", s2, len);

    // --- BORROWING: EXCLUSIVE REFERENCES (`&mut T`) -------------------------------
    //
    // To let a function MODIFY a value without taking ownership, pass `&mut value`.
    // The variable being passed must itself be declared `mut`.
    let mut greeting = String::from("hello");
    append_world(&mut greeting);
    println!("after append: {}", greeting); // "hello, world"

    // --- THE BORROW RULE IN ACTION -------------------------------------------------
    //
    // Shared borrows can coexist: many readers are fine.
    let mut data = String::from("data");
    let r1 = &data; // shared
    let r2 = &data; // shared
    println!("r1 = {}, r2 = {}", r1, r2); // both shared borrows used here
    // r1 and r2 are not used after this point, so their borrows end here (NLL, the
    // "non-lexical lifetimes" model). That frees us to take a mutable borrow below.

    // Now an exclusive borrow. It is allowed because no shared borrow is still live.
    let r3 = &mut data;
    r3.push_str("!");
    println!("r3 = {}", r3);

    // --- STRING vs &str -----------------------------------------------------------
    //
    // `String`   — an owned, growable, heap-allocated UTF-8 buffer.
    // `&str`     — a borrowed "string slice": a view into some UTF-8 text that
    //              someone else owns (a `String`, or a string literal baked into the
    //              binary). String literals have type `&'static str`.
    //
    // A function that only READS text should take `&str`, because it accepts BOTH
    // `&String` (via deref coercion) and string literals, and it never takes ownership.
    let literal = "I am a literal"; // type: &str
    println!("{} / {}", literal, first_word(literal));

    let owned = String::from("owned text");
    // Passing `&owned` where `&str` is expected works via deref coercion.
    println!("{} / {}", owned, first_word(&owned));

    // --- SLICES: VIEWS INTO PART OF A COLLECTION ----------------------------------
    //
    // A slice `&[T]` is a reference to a contiguous run of elements. It borrows rather
    // than copies, so no data is duplicated. `&arr[1..4]` is a slice of indices 1..4.
    let arr = [3, 1, 4, 1, 5, 9];
    let middle = &arr[1..4]; // [1, 4, 1]
    println!("slice = {:?}, sum = {}", middle, sum_slice(middle));
}

// --- FUNCTIONS THAT DEMONSTRATE OWNERSHIP -----------------------------------------

// Takes `text` by value: when this returns, `text` is dropped and its heap memory is
// freed. The caller no longer owns it afterwards.
fn takes_ownership(text: String) {
    println!("takes_ownership got: {}", text);
} // <- `text` dropped here

// Takes a `Copy` type, so it works on a copy and the caller keeps its own value.
fn copies_only(n: i32) {
    println!("copies_only got: {}", n);
}

// Takes a shared reference `&String`: it may READ but not modify, and it does not own
// the data, so nothing is dropped when this returns.
fn calculate_length(text: &String) -> usize {
    text.len() // `.len()` returns the number of BYTES (not necessarily chars) in UTF-8
}

// Takes an exclusive reference `&mut String`: allowed to modify the caller's value.
fn append_world(text: &mut String) {
    text.push_str(", world"); // mutates the string we borrowed
}

// Accepts the flexible `&str`. Returning `&str` here borrows FROM the input, which is
// why the lifetimes line up: the returned slice is valid as long as `text` is.
fn first_word(text: &str) -> &str {
    // `as_bytes` gives a view of the underlying UTF-8 bytes; we search for a space.
    let bytes = text.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // Return the slice from the start up to (not including) the space.
            return &text[..i];
        }
    }
    text // no space: the whole string is the first word
}

// Accepts a slice `&[i32]` and sums it. A slice can be indexed and iterated like an
// array, but its length is not part of the type.
fn sum_slice(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value; // iterator yields `&i32`; dereference to add the `i32`
    }
    total
}
