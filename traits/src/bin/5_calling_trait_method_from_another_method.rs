// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 5)
// ==============================

// Topic: Trait Method calling another Trait Method
// Feel: 
// "Bhai, Trait ke andar bhi dosti hoti hai. Ek method doosre method ko call kar sakta hai.
// Isse code reuse badhta hai aur logic clean rehta hai."

trait Accommodation {
    fn get_name(&self) -> &str;
    fn get_location(&self) -> &str;

    // Default implementation jo doosre methods ko reuse karta hai
    // Feel: "Mera message mere naam aur location se banta hai."
    fn get_summary(&self) -> String {
        format!("🏡 {} is located in {}.", self.get_name(), self.get_location())
    }
}

struct Hotel {
    name: String,
    city: String,
}

impl Accommodation for Hotel {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_location(&self) -> &str {
        &self.city
    }
    
    // Summary humne implement nahi kiya, toh default wala use hoga
    // Jo internally get_name aur get_location call karega.
}

fn main() {
    let taj = Hotel {
        name: String::from("Taj Palace"),
        city: String::from("Delhi"),
    };

    // Yaha magic dekho: get_summary call kiya, lekin usne internally 
    // get_name aur get_location call kar liye!
    println!("{}", taj.get_summary());
}

/*
EXPECTED OUTPUT:
🏡 Taj Palace is located in Delhi.
*/

/*
🧠 Deep Feel:
Agar main get_summary ko override kar doon?
Toh default wala ignore ho jayega. Lekin jab tak override nahi kiya,
wo default logic use karega jo doosre abstract methods par dependent hai.
*/

