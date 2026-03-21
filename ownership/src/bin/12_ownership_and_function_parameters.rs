fn main() {
    let oranges = String::from("Oranges");
    print_my_value(oranges); // let value = oranges;
    // println!("{oranges} is now invalid");
}

fn print_my_value(value: String) {
    println!("Your value is {value}");
} // Here, value goes out of scope and 'drop' is called. Memory is FREED!

// =========================================================
// 🎟️ OWNERSHIP & FUNCTIONS: The One-Way Ticket
// =========================================================

/*
1. THE CONCEPT: Passing is Assigning
------------------------------------
When you pass a variable to a function, it's the SAME as:
`let value = oranges;`

Because `oranges` is a String (Heap data), the ownership MOVES.

2. VISUAL OF THE MOVE INTO FUNCTION
------------------------------------
   [ main() ]                       [ print_my_value() ]
   +-------------+                  +------------------+
   |  (oranges)  |                  |                  |
   | XXX DEAD    | -- (Moved) ----> |  (value)         |
   +-------------+                  | ptr -----------|-----> [ "O R A N G E S" ]
                                    | len: 7           |      (The Heap)
                                    | cap: 7           |
                                    +------------------+

3. THE CONSEQUENCE
-------------------
Once you call `print_my_value(oranges);`, the `oranges` variable
in `main()` is GONE. You cannot use it again.

If you want to keep using it in `main()`, you have two choices:
1. .clone() it before passing (Expensive).
2. Use REFERENCES (&) instead (Recommended - Borrowing).

4. WHAT ABOUT STACK DATA? (i32, bool)
--------------------------------------
If you pass an `i32` to a function, it is COPIED.
The original variable in `main()` stays valid.

SUMMARY:
- Heap data (String) = MOVES to function (Main variable dies).
- Stack data (i32) = COPIES to function (Both stay alive).
*/
