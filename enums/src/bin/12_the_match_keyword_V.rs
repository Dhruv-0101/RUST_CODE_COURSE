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
    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::Whole.drink();

    let milk = Milk::Lowfat(2);
    milk.drink();       // works because `drink(self)` takes ownership
    // milk.drink();       // ❌ ERROR: value moved, cannot use again
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
