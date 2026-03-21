fn main() {
    let person = String::from("Boris");
    let genius = person.clone();

    println!("This is {person}.");
}

// =========================================================
// 🎭 THE "CLONE" METHOD: Making a True Copy
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
   +-------------+  <-- Ownership here

2. THE ACTION: `let genius = person.clone();`
---------------------------------------------
   Unlike `let genius = person;` (which MOVED ownership),
   `.clone()` explicitly asks Rust to:
   1. Allocate NEW memory on the HEAP.
   2. Copy the DATA ("Boris") to the new location.
   3. Create a NEW pointer on the STACK for `genius`.

   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (person)    |               |            |
   | ptr --------|-------------> | "B O R I S"| <-- Original
   | len: 5      |               |            |
   | cap: 5      |               +------------+
   +-------------+               +------------+
   +-------------+               |            |
   | (genius)    |               | "B O R I S"| <-- Clone
   | ptr --------|-------------> |            |
   | len: 5      |               |            |
   | cap: 5      |               +------------+
   +-------------+  <-- Ownership also here

3. WHY USE IT?
--------------
- When you need BOTH variables to be valid at the same time.
- When you want to keep the original data.
- Cost: It is SLOWER than a Move because it copies data.

4. THE "COPY" TRAIT VS "CLONE" TRAIT
--------------------------------------
- COPY: For simple data on the Stack (i32, bool). It's automatic and fast.
- CLONE: For complex data on the Heap (String, Vec). It's manual and slower.

SUMMARY:
- Move: Give ownership away (Fast).
- Copy: Duplicate the value (Stack only).
- Clone: Duplicate the value (Heap allowed).
*/
