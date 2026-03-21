fn main() {
    let person = String::from("Boris");

    drop(person);

    // let genius = person;
}

// =========================================================
// 🗑️ THE "DROP" FUNCTION: Cleaning Up NOW
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

2. THE ACTION: `drop(person);`
------------------------------
   This is like the trash truck coming early!
   - `person` is MOVED into the `drop()` function.
   - The `drop()` function finishes, and it's job is to FREE the memory.

   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (person)    |               |            |
   | XXX DEAD    |               |   FREE!    | <--- (Memory is available again)
   +-------------+               | (Nothing)  |
                                 +------------+

3. WHY USE IT?
--------------
- Normally, Rust drops variables at the end of the scope `}`.
- `drop()` lets YOU decide to clean it up early.
- Once you drop it, you CANNOT use it again (just like a Move).

SUMMARY:
- `drop(x)` = Move into "The Void".
- Memory is freed IMMEDIATELY.
- Variable `x` becomes invalid.
*/
