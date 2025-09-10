// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}

// ┌─────────────────────────────────────────────────────────────┐
// │                  📌 Rust Memory & Globals                    │
// └─────────────────────────────────────────────────────────────┘
//
// In Rust, global data can be declared using `static` or `const`.
//
// 1. `const`
//    - Compile-time constant (inlined wherever used).
//    - Does NOT occupy memory at runtime.
//    - Similar to `static final` in Java.
//    Example:
//        const THRESHOLD: i32 = 10;
//
// 2. `static`
//    - Stored in the **static data segment** of the binary.
//    - Has a single fixed memory location for the entire program.
//    - Value lives for the program's entire lifetime.
//    - Safer form (immutable by default):
//        static LANGUAGE: &str = "Rust";
//    - Mutable form (requires `unsafe` to access):
//        static mut COUNTER: i32 = 0;
//
// 3. Memory Layout Example
//
//    For this code:
//
//        static LANGUAGE: &str = "Rust";
//        const THRESHOLD: i32 = 10;
//
//        fn main() {
//            let n = 16;
//            println!("This is {}", LANGUAGE);
//        }
//
//    Memory layout (simplified):
//
//        +------------------+
//        | .text   (code)   | -> compiled instructions
//        +------------------+
//        | .rodata (consts) | -> "Rust" string literal
//        +------------------+
//        | .data   (statics)| -> pointer for LANGUAGE (&str)
//        +------------------+
//        | .bss    (mut st) | -> mutable statics (if any)
//        +------------------+
//        | Heap             | -> dynamic allocations (Box, Vec, String)
//        +------------------+
//        | Stack            | -> local variables (e.g., n)
//        +------------------+
//
// 4. Java vs Rust Analogy
//    - Java `static final`  ~ Rust `const`
//    - Java `static` field  ~ Rust `static`
//
// ──────────────────────────────────────────────────────────────
//
// ✅ Rule of Thumb:
// - Use `const` for compile-time known values.
// - Use `static` for a single global instance.
// - Use `static mut` only when you *really* need mutable globals 
//   (requires `unsafe`, better replaced with `Atomic` or `Mutex`).
