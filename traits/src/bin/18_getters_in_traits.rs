// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 18)
// ==============================

// Topic: Getters in Traits
// Feel: 
// "Bhai, Rust ke traits field store nahi kar sakte. 
// Lekin hum methods ke zariye field ka data 'Get' kar sakte hain. 
// Isse kehte hain Getter Methods in Traits!"

trait Taxable {
    const TAX_RATE: f64 = 0.07;

    // Ye Getter method hai. 
    // Iski implementation struct khud dega (kyunki struct ke paas data hai).
    fn amount(&self) -> f64;

    // Ye logic default hai, jo internal getter call karega.
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Salary { amount: f64 }
impl Taxable for Salary {
    fn amount(&self) -> f64 {
        self.amount // Asli field idhar hai!
    }
}

struct Bonus { value: f64 }
impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
    fn amount(&self) -> f64 {
        self.value // Yaha field ka naam alag hai, lekin getter same return kar raha hai.
    }
}

fn main() {
    let s = Salary { amount: 60000.0 };
    let b = Bonus { value: 5000.0 };

    println!("--- Getter Pay Report ---");
    println!("Salary Amount: ${}", s.amount());
    println!("Salary Tax: ${}", s.tax_bill());
    println!("Bonus Value: ${}", b.amount());
    println!("Bonus Tax: ${}", b.tax_bill());
}

/*
EXPECTED OUTPUT:
--- Getter Pay Report ---
Salary Amount: $60000
Salary Tax: $4200
Bonus Value: $5000
Bonus Tax: $2500
*/

/*
🧠 Deep Feel:
Kyunki Traits mein fields nahi ho sakte (vo sirf behavior hain), 
isliye Getter methods ek pul (bridge) ki tarah kaam karte hain 
Trait ke behavior aur Struct ke data ke beech mein.
*/

