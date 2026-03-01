fn main() {
    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;

    let thirty_two_bit_signed: i32 = -2147483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    let some_value = 20u16;
    let some_other_value = 20i32;
}
/*
fn main() {
    // Signed integers (can be negative)
    let a: i8  = -128;       // Range: -128 to 127
    let b: i16 = -32_768;    // Range: -32,768 to 32,767
    let c: i32 = -2_147_483_648; // Range: -2,147,483,648 to 2,147,483,647
    let d: i64 = -9_223_372_036_854_775_808; // Range: -9 quintillion to 9 quintillion
    let e: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // Range: ±2^127
    let f: isize = -9223372036854775808; // Range: platform-dependent (same as i32 or i64)

    // Unsigned integers (only positive)
    let g: u8  = 255;        // Range: 0 to 255
    let h: u16 = 65_535;     // Range: 0 to 65,535
    let i: u32 = 4_294_967_295; // Range: 0 to 4.29 billion
    let j: u64 = 18_446_744_073_709_551_615; // Range: 0 to 18 quintillion
    let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // Range: 0 to 2^128-1
    let l: usize = 18_446_744_073_709_551_615; // Range: platform-dependent (same as u32 or u64)

    // Floating-point numbers
    let m: f32 = 3.4028235e38;   // Approx range: ±3.4 × 10^38 (6-7 digits precision)
    let n: f64 = 1.7976931348623157e308; // Approx range: ±1.8 × 10^308 (15-16 digits precision)

    // Char (Unicode scalar value)
    let o: char = '🦀'; // 4 bytes, supports any valid Unicode character

    // Boolean
    let p: bool = true; // true or false

    // Unit type (represents empty value)
    let q: () = (); // Used when no value is returned
}

*/
