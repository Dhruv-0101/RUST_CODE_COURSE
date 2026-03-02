// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 19)
// ==============================

// Topic: Setters in Traits
// Feel: 
// "Bhai, agar 'Get' kar sakte ho toh 'Set' bhi kar sakte ho!
// Trait ek behavior define karta hai: 'Main tera data update kar sakta hoon'.
// Iske liye hume `&mut self` chahiye hota hai."

trait Taxable {
    const TAX_RATE: f64 = 0.07;

    fn amount(&self) -> f64;
    
    // Setter method
    // Feel: "Mera data naya kar do."
    fn set_amount(&mut self, new_amount: f64);

    // Default method jo setter ka use karta hai
    fn apply_bonus(&mut self, bonus: f64) {
        let current = self.amount();
        self.set_amount(current + bonus);
    }

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Salary { amount: f64 }
impl Taxable for Salary {
    fn amount(&self) -> f64 { self.amount }
    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

fn main() {
    let mut my_pay = Salary { amount: 5000.0 };
    
    println!("Initial Tax: ${}", my_pay.tax_bill());

    // Setter call ho raha hai indirectly via apply_bonus
    my_pay.apply_bonus(1000.0);

    println!("After Bonus Amount: ${}", my_pay.amount());
    println!("New Tax: ${}", my_pay.tax_bill());
}

/*
EXPECTED OUTPUT:
Initial Tax: $350
After Bonus Amount: $6000
New Tax: $420
*/

/*
🧠 Deep Feel:
Setter methods allow traits to define logic that MODIFIES the underlying object.
Combined with defaults, you can build complex update logic inside the trait itself, 
as long as the struct provides the basic `set_field` capability.
*/

