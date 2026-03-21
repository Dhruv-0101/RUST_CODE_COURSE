fn main() {
    let mut coffee = String::from("Mocha");
    let a = &mut coffee;
    println!("{a}");
    let b = a;
    println!("{b}");
    // println!("{a}");
}

// =========================================================
// 🏃 THE MUTABLE REFERENCE MOVE: No Duplicates Allowed
// =========================================================

/*
1. THE SCENE: `let b = a;` (where a is &mut T)
-----------------------------------------------
Remember how we said immutable references (`&T`) are `Copy`?
Well, mutable references (`&mut T`) are **NOT** `Copy`.

When you assign `a` to `b`, the reference **MOVES**.

2. WHY DOES IT MOVE?
---------------------
If `&mut` was copied, you would have TWO pointers that can
change the data at the same time.
That would break the "Only One Writer" rule we just learned!

3. VISUAL OF THE MOVE
----------------------
   Step 1: `a` is the only writer.
   Step 2: `let b = a;`

   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (a)         |               |            |
   | XXX DEAD    |               |  "M O C H A"|
   +-------------+               |     ^      |
   +-------------+               |     |      |
   | (b)         |               |     |      |
   | ptr --------|---------------/     |      |
   +-------------+                     +------------+

4. THE CONSEQUENCE
-------------------
After `let b = a;`, you can no longer use `a`. Ownership of the
"Right to Write" has moved to `b`.

SUMMARY:
- &T = Copyable (Many readers).
- &mut T = Moves only (One writer).
- This ensures that at any point in time, there is exactly one
  path to modify the data.
*/
