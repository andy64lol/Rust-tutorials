//! # Tutorial 10 — Structs, Methods, Enums, and `match`
//!
//! This finale ties together Rust's tools for modelling your own data:
//!
//! * **Structs** — group related named fields into one type.
//! * **Tuple structs** — like a tuple, but with a name and named type.
//! * **`impl` blocks** — attach methods and associated functions to a type.
//! * **Enums** — a type that is exactly one of several variants, where each variant
//!   may carry its own data.
//! * **`match`** — exhaustive pattern matching, the natural partner to enums.
//!
//! Rust leans on enums + `match` instead of class hierarchies. Because `match` is
//! exhaustive, the compiler forces you to handle every case — no forgotten branches.

// --- DEFINING A STRUCT ------------------------------------------------------------
//
// A struct has named fields, each with a type. `#[derive(...)]` asks the compiler to
// auto-generate trait implementations: `Debug` lets us print with `{:?}`, `Clone`
// lets us duplicate with `.clone()`, and `PartialEq` lets us compare with `==`.
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

// A TUPLE STRUCT is a named type with unnamed, positionally-accessed fields. Good for
// small wrappers where field names would be noise.
#[derive(Debug)]
struct Point(i32, i32);

// --- ENUMS ------------------------------------------------------------------------
//
// An enum's value is ONE of its variants. Variants can be unit-like (`North`), hold
// tuple data (`Move(i32, i32)`), or hold named fields (`Write { text: String }`).
#[derive(Debug)]
enum Shape {
    Circle(f64),                 // radius
    Rect(u32, u32),              // width, height
    Triangle { base: f64, height: f64 }, // named fields
}

// A classic enum used for nullable values / fallible operations. This is the standard
// library's own `Option`, redeclared here to show how enums express "maybe no value".
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

// --- IMPL BLOCKS: METHODS AND ASSOCIATED FUNCTIONS --------------------------------
//
// `impl Rectangle` adds behaviour to the `Rectangle` type.
impl Rectangle {
    // An ASSOCIATED FUNCTION has no `self`. It is called as `Rectangle::new(...)` and
    // is typically used as a constructor.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height } // field-init shorthand: `width` means `width: width`
    }

    // A METHOD takes `&self` (a shared borrow of the instance). It can read fields but
    // not mutate, and it does not consume the value. Calling it uses `rect.area()`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // A method taking `&mut self` may modify the instance.
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // A method taking `self` by VALUE consumes the instance, so it can only be called
    // once (and you cannot use the rectangle afterwards).
    fn into_square(self) -> Rectangle {
        let side = self.width;
        Rectangle::new(side, side)
    }

    // A method comparing against another rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Point {
    // Tuple struct fields are accessed by index, e.g. `self.0`.
    fn translate(&mut self, dx: i32, dy: i32) {
        self.0 += dx;
        self.1 += dy;
    }
}

// Methods on an enum use the same syntax.
impl Shape {
    // `match self` patterns destructure the variants. `self` here is `&Shape`, so the
    // bindings inside patterns are references; numeric ops auto-deref or copy.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 3.14159 * radius * radius,
            Shape::Rect(width, height) => (*width as f64) * (*height as f64),
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) -> String {
        // `match` arms can produce values; the whole `match` is an expression here.
        match self {
            Shape::Circle(r) => format!("a circle of radius {r}"),
            Shape::Rect(w, h) => format!("a {w}x{h} rectangle"),
            Shape::Triangle { base, height } => format!("a triangle b={base} h={height}"),
        }
    }
}

fn main() {
    // --- CREATING AND USING A STRUCT --------------------------------------------
    //
    // Build an instance with field names, or use our `new` constructor.
    let mut rect = Rectangle { width: 30, height: 50 };
    println!("rect is {:?}", rect); // {:?} uses the derived Debug impl
    println!("area = {}", rect.area()); // call a method with dot syntax

    rect.scale(2); // takes &mut self, so `rect` must be `mut`
    println!("scaled: {:?}, area = {}", rect, rect.area());

    let small = Rectangle::new(10, 10);
    let big = Rectangle::new(40, 40);
    println!("big can hold small: {}", big.can_hold(&small)); // true
    println!("small can hold big: {}", small.can_hold(&big)); // false

    // `into_square` takes `self` by value, consuming `small`. The result is returned.
    let square = small.into_square();
    // println!("{:?}", small); // ERROR: `small` was moved into `into_square`
    println!("square = {:?}", square);

    // --- TUPLE STRUCT ------------------------------------------------------------
    let mut p = Point(1, 2);
    p.translate(3, 4);
    println!("point = ({}, {})", p.0, p.1); // (4, 6)

    // --- ENUMS AND MATCH ---------------------------------------------------------
    let shapes = [
        Shape::Circle(2.0),
        Shape::Rect(3, 4),
        Shape::Triangle { base: 6.0, height: 2.0 },
    ];

    for shape in &shapes {
        // `{:.2}` rounds the float to 2 decimal places.
        println!("{} has area {:.2} ({:?})", shape.describe(), shape.area(), shape);
    }

    // `match` on an enum value, inline, with a catch-all. Prefer listing variants
    // explicitly (the code above does) so the compiler warns you when a new variant
    // is added. `_` is a wildcard that matches anything not listed earlier.
    let peek = Shape::Circle(1.0);
    let kind = match peek {
        Shape::Circle(_) => "round",
        Shape::Rect(_, _) => "boxy",
        // `..` ignores the named fields of the struct-like variant.
        Shape::Triangle { .. } => "pointy",
    };
    println!("the shape is {kind}");

    // --- MATCHING ON OUR Optional ENUM ------------------------------------------
    //
    // `MyOption<T>` is generic; `match` destructures whichever variant we have.
    let present: MyOption<i32> = MyOption::Some(42);
    let missing: MyOption<i32> = MyOption::None;

    // A helper that pattern-matches and returns a fallback for `None`.
    fn value_or_zero(option: &MyOption<i32>) -> i32 {
        match option {
            MyOption::Some(v) => *v,
            MyOption::None => 0,
        }
    }
    println!("present = {}", value_or_zero(&present)); // 42
    println!("missing = {}", value_or_zero(&missing)); // 0

    // `if let` is a shorthand when you only care about ONE variant.
    if let MyOption::Some(v) = present {
        println!("if let unwrapped: {v}");
    }

    // --- PATTERNS CAN DO MORE ----------------------------------------------------
    //
    // `match` also works on numbers, ranges, tuples, and with guards (`if`).
    // This small classifier shows several pattern kinds at once.
    for n in [-5, 0, 7, 42] {
        let category = match n {
            0 => "zero".to_string(),
            x if x < 0 => format!("{x} is negative"),
            1..=9 => format!("{x} is a single digit", x = n),
            _ => format!("{n} is a large number"),
        };
        println!("{category}");
    }
}
