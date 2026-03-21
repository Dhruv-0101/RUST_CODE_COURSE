//go to 22_mutable_reference_conflict.rs to understand the conflict visually
fn main() {
    let mut car = String::from("Red");
    let ref1 = &mut car;
    let ref2 = &mut car;
    // println!("{ref1} and {ref2}");
}
// fn main() {
//     let mut car = String::from("Red");

//     // ---------------------------------------------------------
//     // SCENARIO 1: Multiple Immutable (The "Readers")
//     // ---------------------------------------------------------
//     // MANY people can read the same book at the same time.
//     let ref1 = &car;
//     let ref2 = &car;
//     println!("Readers: {ref1} and {ref2}"); // ✅ OK: Safe to read simultaneously.

//     // ---------------------------------------------------------
//     // SCENARIO 2: Single Mutable (The "Writer")
//     // ---------------------------------------------------------
//     // Only ONE person can write in the book at a time.
//     let ref_mut = &mut car;
//     ref_mut.push_str(" Metallic");
//     println!("Writer: {ref_mut}"); // ✅ OK: Only one writer exists.

//     // ---------------------------------------------------------
//     // SCENARIO 3: The Big Conflict (Read + Write)
//     // ---------------------------------------------------------
//     /*
//     let reader = &car;           // Trying to read...
//     let writer = &mut car;       // ...while someone else is writing!

//     println!("{reader}");        // ❌ ERROR!

//     Why? Imagine the reader is reading "Red", but at that exact
//     microsecond, the writer changes it to "Metallic Blue".
//     The reader would see a corrupted mess!
//     */
//     // ---------------------------------------------------------
//     // SCENARIO 4: Two Writers (Conflict)
//     // ---------------------------------------------------------
//     /*
//     let writer1 = &mut car;
//     let writer2 = &mut car;      // ❌ ERROR!

//     println!("{writer1}");

//     Why? If two people write on the same line at once,
//     the data gets corrupted. Rust says: "One at a time!"
//     */
// }

// // =========================================================
// // 🧠 FEELING THE RULES (The "Library Book" Guide)
// // =========================================================
// /*
//     RULES SUMMARY:
//     1.  EITHER many readers (&T)
//     2.  OR exactly one writer (&mut T)
//     3.  NEVER both at the same time.

//     VISUAL DIAGRAM:

//     [ THE DATA ] <--- & (Reader)  ✅
//                  <--- & (Reader)  ✅

//     [ THE DATA ] <--- &mut (Writer) ✅

//     [ THE DATA ] <--- & (Reader)
//                  <--- &mut (Writer) ❌ (Conflict!)
//                                         The reader might see
//                                         corrupted data.

//     [ THE DATA ] <--- &mut (Writer 1)
//                  <--- &mut (Writer 2) ❌ (Conflict!)
//                                         Who wins? Rust says
//                                         nobody wins, just fix it.
// */
