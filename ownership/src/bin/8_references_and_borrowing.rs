fn main() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference);
    println!("{}", my_heap_value);
}

// =========================================================
// 🔗 REFERENCES & BORROWING: Sharing Without Moving
// =========================================================

/*
1. THE PROBLEM: Moves are Expensive
-----------------------------------
If we pass a String to a function, it MOVES:

fn print_string(s: String) { println!(s); } // s is GONE after this

let my_string = String::from("Hello");
print_string(my_string); // my_string is now INVALID!

We want to "borrow" it instead of giving it away.

2. THE SOLUTION: The Reference (&)
-----------------------------------
Instead of moving the data, we pass a "Pointer" (Reference) to it.

fn print_string(s: &String) { println!(s); } // s is just a reference

let my_string = String::from("Hello");
print_string(&my_string); // Pass the address, not the data
print_string(&my_string); // We can use it again!

3. THE RULES (The Borrowing Rules)
----------------------------------
Rust has strict rules to prevent "Data Races" (multiple people writing at once):

Rule 1: You can have:
        - EITHER many "Immutable References" (&T)
        - OR exactly ONE "Mutable Reference" (&mut T)

Rule 2: You cannot have both at the same time.

4. THE VISUAL LOOK
-------------------

Scenario A: Multiple Readers (OK)
   [ THE STACK ]
   +-------------+
   | (my_string) | <--- Original Data
   | ptr --------|-------------> [ "H E L L O" ]
   | len: 5      |               (The Heap)
   | cap: 5      |
   +-------------+
   +-------------+
   | (ref1)      | <--- Borrowed (Read Only)
   | ptr --------|-------------
   +-------------+
   +-------------+
   | (ref2)      | <--- Borrowed (Read Only)
   | ptr --------|-------------
   +-------------+

Scenario B: One Writer (OK)
   [ THE STACK ]
   +-------------+
   | (my_string) | <--- Original Data
   | ptr --------|-------------> [ "H E L L O" ]
   | len: 5      |               (The Heap)
   | cap: 5      |
   +-------------+
   +-------------+
   | (ref_mut)   | <--- Borrowed (Read & Write)
   | ptr --------|-------------
   +-------------+
   (No other references allowed here!)

Scenario C: The Error (NOT OK)
   [ THE STACK ]
   +-------------+
   | (my_string) | <--- Original Data
   | ptr --------|-------------> [ "H E L L O" ]
   | len: 5      |               (The Heap)
   | cap: 5      |
   +-------------+
   +-------------+
   | (ref1)      | <--- Borrowed (Read)
   | ptr --------|-------------
   +-------------+
   +-------------+
   | (ref_mut)   | <--- Borrowed (Write)
   | ptr --------|-------------
   +-------------+
   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
         !!! ERROR !!!
   Rust says: "You can't read AND write at the same time!"

5. SUMMARY
----------
- Reference (&): A temporary "view" of the data.
- Borrowing: Lending data without giving up ownership.
- Immutable Ref (&): Read-only. Many allowed.
- Mutable Ref (&mut): Read & Write. Only ONE allowed.
*/
