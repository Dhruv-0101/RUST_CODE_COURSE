fn main() {
    let ice_cream = "Cookies and Cream"; // This is a &str (a reference to a string literal)
    let dessert = ice_cream; // This is a COPY, not a MOVE!

    println!("{ice_cream} {dessert}."); // Both are valid because &str is Copyable.
}

// =========================================================
// 🔄 COPY TRAIT WITH REFERENCES: Why it works!
// =========================================================

/*
1. THE CORE RULE: POINTERS ARE JUST NUMBERS
-------------------------------------------
Even if the data (like a 1GB String) is NOT copyable, the
REFERENCE (the address) is always just a small, fixed-size number.

In 64-bit systems, a pointer is exactly 8 bytes.
Copying 8 bytes is trivial and safe for the CPU.

2. VISUAL OF THE COPY
----------------------
   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   |  (ice_cream)|               |            |
   | ptr --------|-------------> | "C O O K I E S" |
   +-------------+               |            |
   |  (dessert)  |               |            |
   | ptr (same!) |---------------/            |
   +-------------+               +------------+

   When we do `let dessert = ice_cream;`:
   - Rust just copies the "Address Card" (Pointer).
   - Now we have TWO cards pointing to the SAME house.

3. SPECIAL CASE: &str
---------------------
String literals (like "Cookies and Cream") are of type `&str`.
Since they are already references, they follow the "References are Copy" rule.

4. WHAT ABOUT &String?
-----------------------
let s = String::from("Hello");
let r1 = &s;
let r2 = r1; // This is ALSO a copy of the pointer!

Because r1 and r2 are just references, you can have as many
copies of them as you want (as long as they are immutable).

5. SUMMARY:
------------
- T (String) = Move by default.
- &T (Reference) = Copyable! (Because addresses are small).
- You can copy a "View" of the data as many times as you like.
*/
