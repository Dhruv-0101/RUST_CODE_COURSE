#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
}
//match the value also
impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

fn main() {
    // 🥛 TEMPORARY GLASSES (Works Every Time!)
    Milk::Lowfat(2).drink(); // New glass 1 -> Drunk & Gone.
    Milk::Lowfat(1).drink(); // New glass 2 -> Drunk & Gone. 
    Milk::Whole.drink();     // New glass 3 -> Drunk & Gone.

    /*
    🤔 Why does THIS work? (The Fridge Analogy)
    -----------------------------------------
    - `Milk::Lowfat(2)` ek 'Temporary' object hai. 
    - Ye bilkul aisa hai ki aapne fridge se ek NAYA glass nikala...
    - Use piya (`.drink()`) aur wo khatam ho gaya.
    - Agli line mein aap fridge se ek aur NAYA glass (`Lowfat(1)`) nikal rahe ho. 
    - Isliye ye hamesha chalega! Aap har bar naye object ke sath kaam kar rahe ho.

    But in the next part... 
    Aapne `let milk = ...` likh kar glass ko ek 'Naam' de diya hai. 
    Ab aap usi SAME glass ki baat kar rahe ho.
    */

    let milk = Milk::Lowfat(2);
    println!("{:#?}", milk);
    milk.drink(); // works because `drink(self)` takes ownership
    // milk.drink();       // ❌ ERROR: value moved, cannot use again

    // =========================================================
    // 🥛 THE "GLASS OF MILK" ANALOGY (Ownership Move)
    // =========================================================
    /*
    1. THE PROBLEM: `fn drink(self)`
    -------------------------------
    Notice that the method signature is `drink(self)`, NOT `drink(&self)`.
    In Rust, when you pass `self` (without &), you are MOVING ownership.

    2. THE ANALOGY:
    ---------------
    - Imagine `milk` is a real glass of milk.
    - `drink(self)` means you are actually DRINKING it.
    - Once you drink a glass of milk, is it still there? NO. 
    - It's gone! You have consumed it.

    3. WHY THE SECOND CALL FAILS:
    -----------------------------
    - Line 30: `milk.drink()` -> You "drank" the milk. The variable `milk` 
      is now EMPTY (it no longer owns any data).
    - Line 31: `milk.drink()` -> You are trying to drink from an 
      empty glass that's already in your stomach! ❌ ERROR.

    4. THE SOLUTION (If you wanted to reuse it):
    --------------------------------------------
    If you wanted to check the milk without consuming it, you would use:
    `fn check(&self)` -> This is like just LOOKING at the glass.
    Looking at it doesn't make it disappear!

    SUMMARY:
    - self = Consume (Move).
    - &self = Look/Borrow (No move).
    - Ek baar 'drink(self)' kar liya, toh dobara nahi kar sakte!
    */
}
// 🧠 Key Concept:
// Receiver    	      Meaning	               Ownership	         When to use
// self	            Takes ownership	           Moves it	      When you consume the value
// &self	        Borrows immutably	       No move	      When you just read/check data
// &mut self  	    Borrows mutably	           No move	      When you want to modify it

/*
✅ Your Two Examples
1️⃣ &self in OnlineOrderStatus::check()

fn check(&self) {
    match self {
        OnlineOrderStatus::Delivered => println!("Your item has arrived"),
        other_status => println!("Your item is {other_status:?}"),
    }
}
&self = borrow only, so the enum value can still be used after calling .check().
You're just checking status, not changing or consuming it.
Safe and efficient.


2️⃣ self in Milk::drink()
fn drink(self) {
    match self {
        Milk::Lowfat(2) => println!("Delicious, 2% is my favorite!"),
        Milk::Lowfat(percent) => println!("You've got the lowfat {percent} percent version!"),
        Milk::Whole => println!("You've got the whole milk!"),
    }
}
self = takes ownership.
You're consuming the enum value (not borrowing), because after .drink() is called, the value is no longer usable.
Good if you don’t need to reuse the enum again.


let milk = Milk::Lowfat(2);
milk.drink();       // works because `drink(self)` takes ownership
milk.drink();       // ❌ ERROR: value moved, cannot use again

let status = OnlineOrderStatus::Shipped;
status.check();     // OK
status.check();     // OK again, because value wasn't moved

*/
