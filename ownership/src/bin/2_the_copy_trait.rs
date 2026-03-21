fn main() {
    let time = 2025;
    let years = time;

    println!("The time is {time}. It is the year {years}.");
}
// A trait is a way to define shared behavior in Rust.
/*trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}
Here:

Animal is a trait — like saying "anything that is an animal must know how to make_sound()".

Dog implements that trait by defining what make_sound() means for a dog.

2025
2025
stack*/

// =========================================================
// 🧠 STACK VS HEAP: The "Feel" Guide
// =========================================================

/*
1. THE STACK (The Organized Bookshelf)
--------------------------------------
Analogy: Think of a stack of dinner plates.
- How it works: LIFO (Last In, First Out). You put a plate on top, you take a plate from the top.
- Speed: Super FAST! The CPU doesn't have to search for space; it knows exactly where the top is.
- Size: Every piece of data MUST have a fixed, known size at compile time.
  Example: i32, bool, f64, fixed-size arrays.

let x = 5;  <-- This '5' lives on the STACK. It's safe, fast, and simple.

2. THE HEAP (The Luxury Restaurant)
------------------------------------
Analogy: Think of being seated at a restaurant.
- How it works: You walk in and say "We are 4 people" (allocating space).
  The host finds a table and gives you the table number (a POINTER).
- Speed: SLOWER. The allocator has to search for a big enough "empty spot" in the heap.
- Access: To get the data, you have to follow the pointer (the address) from the stack to the heap.
- Size: Used for data that can GROW or SHRINK (like a String or a Vector).

let s = String::from("Hello");
<-- "Hello" is on the HEAP.
<-- The POINTER (address), length, and capacity are on the STACK.

3. WHY DOES RUST CARE? (Ownership)
----------------------------------
- Stack data is easy: When the function ends, the stack just "pops" and the data is gone. Clean!
- Heap data is hard: If you forget to "clean up" (free) the table at the restaurant, the restaurant
  runs out of space (Memory Leak).
- Ownership's main job: To manage who "owns" that table on the HEAP so Rust knows exactly
  when to clean it up automatically.

Summary:
- Stack = Fast, Fixed size, Automatic cleanup.
- Heap = Flexible size, Slower, Needs Ownership to manage cleanup.

4. THE VISUAL LOOK (Memory Diagram)
-----------------------------------

    [ THE STACK ]                      [ THE HEAP ]
    Fixed size gems                     The vast ocean
    +---------------+                +-------------------+
    | let x = 5     |                |                   |
    | (4 bytes)     |                |                   |
    +---------------+                |                   |
    | let name      | -------------> |  "A N T I G R A V I T Y" |
    | (Pointer,     |    (Pointer)   |  (The actual data)|
    | Len, Cap)     |                +-------------------+
    +---------------+

5. STATIC VS DYNAMIC (Example)
------------------------------

A) STATIC (Stack Only)
   let age: i32 = 25;
   -> Rust knows it's EXACTLY 4 bytes.
   -> It stays on the Stack. Fast and easy to copy.

B) DYNAMIC (Stack + Heap)
   let mut bio = String::from("Rustacean");
   -> If you add more text (" is learning"), the size CHANGES.
   -> Rust can't keep it on the Stack because the Stack needs "Fixed Size".
   -> So, it puts the text on the HEAP and keeps the "GPS Coordinates" (Pointer) on the Stack.

       [ THE STACK ]                   [ THE HEAP ]
    (Fixed-size values)            (Dynamic data)
    +---------------+              +--------------------+
    | age = 25      | <--- Seedha value stack pe hai
    +---------------+              |                    |
    | name:         |              | "Antigravity"      |
    |  ptr -------- |------------> | (Actual text yahan |
    |  len: 11      |              |  store hota hai)   |
    |  cap: 11      |              +--------------------+
    +---------------+

*/
