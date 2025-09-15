// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    // an f32 literal (use `_f32` suffix to force 32-bit float).
    let decimal = 65.4321_f32;

    // Error! No implicit conversion: Rust won't implicitly convert float -> integer.
    // let integer: u8 = decimal;
    // ^-- commented out: this does not compile because Rust requires an explicit cast

    // Explicit conversion using `as`.
    // For floats -> ints: fractional part is truncated toward zero,
    // and if out of range or NaN, the value is saturated to the target bounds.
    let integer = decimal as u8;

    // Convert numeric value 65 -> Unicode scalar value 'A'.
    // Note: Rust `char` is a 4-byte Unicode scalar, not Java's 2-byte `char`.
    let character = integer as char;

    // Error! You cannot cast f32 directly to `char`.
    // You must first convert float -> integer, then integer -> char.
    // let character = decimal as char;
    // ^-- commented out: invalid cast (only integer -> char is allowed)

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value fits into the new type

    // 1000 already fits in a u16 -> prints 1000
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 -> u8 keeps only the lowest 8 bits => 232
    // (1000 - 256 - 256 - 256 = 232)
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 in i8 has bit pattern 0xff -> as u8 becomes 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, truncation to u8 is equivalent to modulus 256
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the bitwise pattern is interpreted as signed.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as i8 overflows the positive range -> bit pattern is 0x80 -> interpreted as -128
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 bit pattern as i8 is -24 (232 - 256 = -24)
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, float->int via `as` saturates to min/max; NaN -> 0.
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // Unsafe unchecked conversion: avoids saturating checks and uses LLVM conversion.
    // This is faster but may produce unsound/undefined results for out-of-range or NaN inputs.
    unsafe {
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
