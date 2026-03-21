fn main() {
    // ---------------------------------------------------------
    // 1. THE STACK VALUE (The "Box")
    // ---------------------------------------------------------
    let my_stack_value = 2;

    // ---------------------------------------------------------
    // 2. THE REFERENCE (The "GPS Coordinates" - &)
    // ---------------------------------------------------------
    // & means: "Find the address of my_stack_value"
    let my_reference = &my_stack_value;

    println!("--- UNDERSTANDING THE REFERENCE ---");
    // {:p} prints the actual MEMORY ADDRESS (The Hex code)
    println!("The GPS Address (Pointer) is: {:p}", my_reference);

    // Rust is smart: if you print a reference with {}, it automatically
    // "dereferences" it for you to show the value. This creates confusion!
    println!(
        "Smart Print (shows the value even though it's a ref): {}",
        my_reference
    );

    // ---------------------------------------------------------
    // 3. THE DEREFERENCE (Visiting the Box - *)
    // ---------------------------------------------------------
    // * means: "Go to the address and bring the value back"
    let actual_value = *my_reference;

    println!("\n--- UNDERSTANDING THE DEREFERENCE ---");
    println!("Explicitly visiting the address via *: {}", actual_value);

    // ---------------------------------------------------------
    // 4. COMPARISON (Feel the difference)
    // ---------------------------------------------------------

    let a = 10;
    let b = &a; // b is just the address of a

    // This is valid: Comparing value with dereferenced value
    if a == *b {
        println!("\nSUCCESS: Value 'a' matches the data inside address 'b'!");
    }

    // ---------------------------------------------------------
    // 5. THE HEAP EXAMPLE (String - The "Toyota")
    // ---------------------------------------------------------
    let my_car = String::from("Toyota");
    let car_ref = &my_car; // car_ref is the ADDRESS on the stack

    println!("The car is {car_ref}");

    // {:p} on a String reference shows where the POINTER is on the Stack
    println!("\n--- STRING (HEAP) EXAMPLE ---");
    println!("Car Reference Address: {:p}", car_ref);

    // *car_ref would normally be the actual string "Toyota"
    // BUT! You can't do: let copy = *car_ref;
    // Because String (str) size is Unknown, and you can't 'own' it without the pointer.

    if *car_ref == "Toyota" {
        println!("Checking car model: It's a Toyota!");
    }
}

// =========================================================
// 🧠 FINAL SUMMARY TO CLEAR CONFUSION
// =========================================================
/*
    TERM          | OPERATOR | ANALOGY             | RESULT
    --------------|----------|---------------------|------------------
    Reference     |    &     | Taking a photo of   | Memory Address
                  |          | the house's address | (e.g. 0x7ffd...)
    ---------- ---|----------|---------------------|------------------
    Dereference   |    *     | Opening the door of | Actual Data
                  |          | that house          | (e.g. 2, 10, "Hi")

    Why is it confusing?
    Because Rust's `println!` is too "helpful". It looks at a reference (&)
    and says "I know you wanted to see the value, not the address",
    so it does the `*` for you automatically.
*/
