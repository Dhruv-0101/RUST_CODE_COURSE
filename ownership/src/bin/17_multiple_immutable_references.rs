fn main() {
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);
}

// =========================================================
// 📚 THE LIBRARY ANALOGY: Multiple Readers are OK
// =========================================================

/*
1. THE SCENE: `let ref1 = &car; let ref2 = &car;`
------------------------------------------------
Rust allows you to create as many immutable references as you
want.

Why? Because nobody is CHANGING the data.

2. THE ANALOGY
---------------
Imagine a book in a library:
- Many people can read the same book at the same time.
- As long as nobody is writing on the pages or tearing them
  out, there is no conflict.
- Everyone sees the same "Red" car.

3. VISUAL OF MULTIPLE BORROWS
------------------------------
   [ THE STACK ]                 [ THE HEAP ]
   +-------------+               +------------+
   | (car)       | ------------> |  "R E D"   |
   +-------------+               +------------+
   | (ref1)      | -----/               ^
   +-------------+                      |
   | (ref2)      | ---------------------/
   +-------------+
   | (ref3)      | --------------------/
   +-------------+

SUMMARY:
- Many &T = Perfectly safe.
- Rust's motto: "Sharing is caring (as long as you don't touch!)"
*/
