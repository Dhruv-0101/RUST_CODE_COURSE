//give and take ownership
fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    // current_meal = add_sugar(current_meal);
    // add_salt();
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

// =========================================================
// 🔄 TAKE AND GIVE BACK: The Ownership Relay
// =========================================================

/*
1. THE PATTERN: Receive -> Modify -> Return
-------------------------------------------
Sometimes you want a function to change your data, but you
still want to "own" it after the function is done.

If you don't use references, you have to do a RELAY:
`current_meal = add_flour(current_meal);`

2. VISUAL OF THE RELAY
-----------------------
Step A: `main` sends `current_meal` to `add_flour`. (`main` loses ownership)
Step B: `add_flour` modifies it.
Step C: `add_flour` returns it. (`main` gets ownership back!)

   [ main ]  --- (A: Move In) ---> [ add_flour ]
                                        |
                                   (B: Modify)
                                        |
   [ main ] <--- (C: Move Out) --- [ add_flour ]

3. WHY WOULD YOU DO THIS?
--------------------------
If you don't know about references (`&`) yet, this is the only
way to let a function modify your heap data and still keep
using it in `main()`.

Wait... is there a better way?
YES! Borrowing with References (&mut). It's much cleaner
because you don't have to keep re-assigning the variable.

SUMMARY:
- Returning a value that was passed in is a valid way to move
  ownership back to the caller.
- It's like lending a book and getting a different edition back.
*/
