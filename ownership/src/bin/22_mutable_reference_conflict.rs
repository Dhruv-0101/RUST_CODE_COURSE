/*
   fn main() {
       let mut car = String::from("Red");
       let ref1 = &mut car;
       let ref2 = &mut car;
       println!("{ref1} and {ref2}"); // ❌ ERROR: ref1 is now INVALID!
   }
*/

// =========================================================
// 🚫 THE "SINGLE WRITER" CONFLICT: Visual Guide
// =========================================================

/*
1. THE SCENE: `let mut car = String::from("Red");`
--------------------------------------------------
   `car` is the house. It's mutable, so we can change it.

2. THE FIRST BORROW: `let ref1 = &mut car;`
-------------------------------------------
   `ref1` takes the ONLY "Mutable Key" to the house.

   [ THE STACK ]                   [ THE HEAP ]
   +-------------+                 +------------+
   | (ref1)      | -- (Has Key) -> |  "R E D"   |
   +-------------+                 +------------+

3. THE SECOND BORROW: `let ref2 = &mut car;`
--------------------------------------------
   Rust says: "Wait! You want another Mutable Key?
   You must throw away the first one (ref1) to get the second one (ref2)."

   [ THE STACK ]                   [ THE HEAP ]
   +-------------+                 +------------+
   | (ref1)      | -- XXX DEAD --> |  "R E D"   |
   +-------------+                 +------------+
   +-------------+                      ^
   | (ref2)      | -- (Has Key) --------/
   +-------------+

4. THE CRASH: `println!("{ref1} and {ref2}");`
----------------------------------------------
   You are trying to use `ref1` (which is now DEAD) and `ref2` (which is ALIVE).
   Rust says: "Bhai, ref1 toh khatam ho gaya jab ref2 ne ownership li!
   Tum do writers ek saath nahi rakh sakte."

SUMMARY:
- &mut is EXCLUSIVE.
- Jab `ref2` banta hai, toh `ref1` ka 'permission' khatam ho jata hai.
- Isiliye aap dono ko ek saath use nahi kar sakte.

5. THE SEPARATE USE EXCEPTION (NLL - Non-Lexical Lifetimes)
-----------------------------------------------------------
If you use `ref1` and then FINISH with it, you can make `ref2`.
This is because Rust is smart enough to see that their
"lives" (lifetimes) do NOT overlap.

Example that WORKS:
   let mut car = String::from("Red");

   let ref1 = &mut car;
   println!("{ref1}"); // ref1 is used here for the LAST time. ✅

   let ref2 = &mut car; // NOW it's safe to take the key for ref2.
   println!("{ref2}");
*/

fn main() {
    let mut car = String::from("Red");

    // This WORKS because they are used sequentially!
    let ref1 = &mut car;
    println!("First use: {ref1}"); // ref1's life ends here.

    let ref2 = &mut car;
    println!("Second use: {ref2}"); // ref2's life starts here.

    println!("\nConclusion: References can overlap as long as they are NOT in use together.");

    // ---------------------------------------------------------
    // 6. WHY THE USER'S EXAMPLES DON'T WORK (The Overlap)
    // ---------------------------------------------------------
    /*
    Example 1:
    let ref1 = &mut car; // Window 1 starts...
    let ref2 = &mut car; // ❌ ERROR: Window 2 starts while Window 1 is still open!
    println!("{ref2}");
    println!("{ref1}"); // ...Window 1 ends here.

    Why? Because 'ref1' is used AFTER 'ref2' is born.
    So for a moment, TWO people had the 'Edit' key.
    Rust doesn't allow this even for a millisecond!
    */

    /*
        Example 2:
        let ref1 = &mut car; // Window 1 starts...
        let ref2 = &mut car; // ❌ ERROR: Again, Window 2 starts while Window 1 is open.
        println!("{ref1}");
        println!("{ref2}"); // Window 2 ends here.

        Order doesn't matter. If their "Life Windows" overlap, it's a conflict!


        let ref1 = &mut car; // [Window 1 starts]
    let ref2 = &mut car; // ❌ Error! Window 2 start nahi ho sakti...
    println!("{ref2}");
    println!("{ref1}");  // ...kyunki Window 1 yahan tak khuli hai.


    let ref1 = &mut car; // [Window 1 starts]
    /* Yahan ref2 banane ki koshish */
    let ref2 = &mut car; // ❌ Wo yahan hi error de dega.
    println!("{ref1}");  // Kyunki ye abhi zinda hai.
    println!("{ref2}");

        */
}
