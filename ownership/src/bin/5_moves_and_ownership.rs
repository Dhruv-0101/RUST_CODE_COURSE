fn main() {
    let person = String::from("Boris");
    println!("My name is {person}");

    let genius = person;

    // println!("My name is {person}");
}

// =========================================================
// 🔄 THE "MOVE" CONCEPT: How Ownership Shifts
// =========================================================

/*
1. THE SCENE: `let person = String::from("Boris");`
--------------------------------------------------
   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (person)    |               |            |
   | ptr --------|-------------> | "B O R I S"|
   | len: 5      |               |            |
   | cap: 5      |               +------------+
   +-------------+

2. THE ASSIGNMENT: `let genius = person;`
-----------------------------------------
   Rust DOES NOT copy the "Boris" text on the HEAP (too slow!).
   Rust JUST copies the 'ptr, len, cap' to the new variable:

   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (person)    |               |            |
   | XXX INVALID |               | "B O R I S"|
   +-------------+               |     ^      |
   +-------------+               |     |      |
   | (genius)    |               |     |      |
   | ptr --------|---------------/     |      |
   | len: 5      |                     |      |
   | cap: 5      |                     |      |
   +-------------+                     +------------+

3. WHY DID IT MOVE? (The "Double-Free" Prevention)
-------------------------------------------------
- When a variable goes out of scope, Rust automatically cleans its data (Free).
- If BOTH `person` and `genius` were valid, Rust would try to free "Boris" twice!
- That would crash your program.
- So, Rust says: "I will MOVE the ownership to 'genius' and kill 'person'."

SUMMARY:
- For Stack values (i32), data is copied.
- For Heap values (String), ownership is MOVED.
- After a move, the old variable is GONE. You can't use it.
*/
