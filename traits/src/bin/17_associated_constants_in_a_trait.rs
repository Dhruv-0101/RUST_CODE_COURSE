// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 17)
// ==============================

// Topic: Associated Constants in Traits
// Feel:
// "Bhai, Trait sirf methods ki list nahi hai.
// Ye constants (fixed values) bhi apne paas rakh sakta hai.
// Jaise har taxable cheez ka ek standard TAX_RATE hota hai!"

trait Taxable {
    // 1. Default Associated Constant
    // Feel: "Sabke liye standard 7% tax"
    const TAX_RATE: f64 = 0.07;

    fn tax_bill(&self) -> f64;
}

struct Salary {
    amount: f64,
}
impl Taxable for Salary {
    // Standard tax rate use karenge
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE // Self::TAX_RATE refers to the constant in the trait
    }
}

struct LuxuryGift {
    value: f64,
}
impl Taxable for LuxuryGift {
    // 2. Overriding the Constant
    // Feel: "Luxury items pe 50% tax, koi raham nahi!"
    const TAX_RATE: f64 = 0.50;

    fn tax_bill(&self) -> f64 {
        self.value * Self::TAX_RATE
    }
}

fn main() {
    let income = Salary { amount: 50000.0 };
    let gift = LuxuryGift { value: 10000.0 };

    println!("--- Tax Report ---");
    println!("Salary Tax: ${}", income.tax_bill());
    println!("Luxury Gift Tax: ${}", gift.tax_bill());
}

/*
EXPECTED OUTPUT:
--- Tax Report ---
Salary Tax: $3500
Luxury Gift Tax: $5000
*/

/*
🧠 Deep Feel:
Associated constants ka fayda ye hai ki aap behavior ke saath-saath
configuration bhi trait level pe define kar sakte ho.
Types ko forced feel nahi hota, wo chahe toh override kar sakte hain (jaise luxury gift ne kiya).
*/
