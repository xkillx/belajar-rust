fn main() {
    // Declare a variable binding without initialization
    let a_binding;

    {
        let x = 2; // x is scoped only to this block

        // Initialize a_binding with x * x
        a_binding = x * x; // a_binding = 4
    }

    // Safe: a_binding has been initialized
    println!("a binding: {}", a_binding);

    // Declare another binding without initialization
    let another_binding;

    // ❌ Error: cannot use before initialization
    // println!("another binding: {}", another_binding);

    // Now we assign it
    another_binding = 1;

    // ✅ Safe: now it's initialized
    println!("another binding: {}", another_binding);
}
