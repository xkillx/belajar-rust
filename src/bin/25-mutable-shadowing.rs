fn main() {
    // Define a mutable integer (can be reassigned later)
    let mut _mutable_integer = 7i32;

    {
        // Shadowing: create a *new* variable with the same name,
        // but this one is immutable by default
        let _mutable_integer = _mutable_integer;

        // ❌ Compile error: cannot assign to immutable variable
        // _mutable_integer = 50;
        // FIXME: comment this out to make code compile

        // Shadowed `_mutable_integer` ends here (scope ends)
    }

    // ✅ Back to the original mutable variable
    _mutable_integer = 3;
}
