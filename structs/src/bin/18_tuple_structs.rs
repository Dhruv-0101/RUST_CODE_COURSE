// Hours, minutes
struct ShortDuration(i32, i32);
// Years, months
struct LongDuration(i32, i32);

fn main() {
    let work_shift = ShortDuration(8, 30);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(era);
    // accept_tuple(era);

    // let work_shift = (8, 0);
    // let era = (5, 3);
    // go_to_work(work_shift);
    // go_to_work(era);//no error
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}

// fn accept_tuple(length: (u32, u32)) {}

/*
🧠 Code Recap
rust
struct ShortDuration(i32, i32);
struct LongDuration(i32, i32);

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}

fn main() {
    let era = LongDuration(5, 3);
    go_to_work(era); // ❌ This should cause a compile-time error!
}
❌ Why This Is an Error
go_to_work expects a parameter of type ShortDuration.

You're passing era, which is of type LongDuration.

Even though both structs have the same fields (i32, i32), they are distinct types in Rust.

Rust is strictly type-safe — it does not allow implicit conversion between different struct types, even if their fields match.

🧪 What the Error Looks Like
If you try to compile this, Rust will say something like:

Code
error[E0308]: mismatched types
  --> main.rs:XX:XX
   |
XX |     go_to_work(era);
   |                ^^^ expected struct `ShortDuration`, found struct `LongDuration`

   
✅ Why This Works (Tuple Version)
rust
let work_shift = (8, 0);
let era = (5, 3);
go_to_work(work_shift); // ✅ This works if go_to_work accepts a tuple
If you change go_to_work to accept a tuple like this:

rust
fn go_to_work(length: (i32, i32)) {
    println!("Passing time {} hours {} minutes", length.0, length.1);
}
Then both (8, 0) and (5, 3) are just plain tuples — no custom type — so Rust allows it.
*/


/*
// fn main() {
//     let red: (u8, u8, u8) = (255, 0, 0);
//     set_bg_color(red);

//     let point: (u8, u8, u8) = (200, 0, 0);
//     move_point(red);
// }

// fn set_bg_color(color: (u8, u8, u8)) {
//     println!(
//         "Setting bg color R={}, G={} ,B={}",
//         color.0, color.1, color.2
//     );
// }

// fn move_point(point: (u8, u8, u8)) {
//     println!(
//         "Point moved from: (X={}, Y={}, Z={})",
//         point.0, point.1, point.2
//     );
// }


struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

fn main() {
    let red: Color = (255, 0, 0);
    set_bg_color(red);

    let point: Point = (200, 0, 0);
    move_point(red);
}

fn set_bg_color(color: Color) {
    println!(
        "Setting bg color R={}, G={} ,B={}",
        color.0, color.1, color.2
    );
}

fn move_point(point: Point) {
    println!(
        "Point moved from: (X={}, Y={}, Z={})",
        point.0, point.1, point.2
    );
}

*/