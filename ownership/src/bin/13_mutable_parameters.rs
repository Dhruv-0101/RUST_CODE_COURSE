//give ownership
fn main() {
    let burger = String::from("Burger");
    add_fries(burger);
    // println!("{burger}");
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
} // Ownership of 'meal' ends here. Memory is dropped.

// =========================================================
// ✍️ MUTABLE PARAMETERS: New Owner, New Rules
// =========================================================

/*
1. THE SCENE: `add_fries(burger);`
----------------------------------
`burger` was immutable in main(). But when it moves to `add_fries`,
ownership is handed over completely.

2. THE "NEW OWNER" CONCEPT
---------------------------
When the function `add_fries` gets the ownership, it can decide
how it wants to keep it. By saying `mut meal`, the function is
saying: "I am the new owner, and I want to change this data."

3. VISUAL OF THE RE-OWNERSHIP
-----------------------------
   [ main() ]                       [ add_fries() ]
   +-------------+                  +------------------+
   | (burger)    | -- (Moved) ----> | (mut meal)       |
   | Immutable   |                  | Mutable (Yay!)   |
   +-------------+                  |                  |
                                    | "Burger" ------> |
                                    | "Burger & Fries" |
                                    +------------------+

4. WHY IS THIS ALLOWED?
-----------------------
Because the original `burger` in main() is now DEAD/INVALID.
There is no one else looking at this data, so it's perfectly safe
for the function to change it however it wants.

SUMMARY:
- When you MOVE a variable, you can change its mutability.
- `mut meal` lets the function modify the data it now owns.
- The original variable doesn't care (because it's gone!).
*/
