// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 20)
// ==============================

// Topic: Supertraits (Trait Inheritance)
// Feel:
// "Bhai, Supertrait matlab 'Shart' (Requirement).
// Agar tum 'Taxable' banna chahte ho, toh pehle tumhe 'Investment' hona padega.
// Bina Investment ke Taxable ka koi wajood nahi!"

trait Investment {
    fn amount(&self) -> f64;
}

// Taxable: Investment matlab...
// "Taxable implement karne se pehle, Investment implement karna mandatory hai."
trait Taxable: Investment {
    fn tax_bill(&self) -> f64 {
        self.amount() * 0.10 // Investment ka method use kar raha hai!
    }
}

struct Gold {
    weight_in_grams: f64,
    price_per_gram: f64,
}

// Step 1: Pehle Investment implement karo
impl Investment for Gold {
    fn amount(&self) -> f64 {
        self.weight_in_grams * self.price_per_gram
    }
}

// Step 2: Ab Taxable implement kar sakte ho
impl Taxable for Gold {}

fn main() {
    let my_gold = Gold {
        weight_in_grams: 100.0,
        price_per_gram: 50.0,
    };

    println!("--- Investment Report ---");
    println!("Total Value: ${}", my_gold.amount());

    // Taxable method call kar rahe hain jo internally Investment call karta hai
    println!("Tax to pay: ${}", my_gold.tax_bill());
}

/*
EXPECTED OUTPUT:
--- Investment Report ---
Total Value: $5000
Tax to pay: $500
*/

/*
🧠 Deep Feel:
Supertraits classical inheritance jaisa nahi hai.
Ye sirf ek 'Requirement' hai. Rust compiler verify karta hai ki jo type
sub-trait implement kar raha hai, usne parent-trait bhi implement kiya ya nahi.
Isse code modular rehta hai aur hum assumptions le sakte hain (like 'tax_bill' assumes 'amount' exists).
*/
