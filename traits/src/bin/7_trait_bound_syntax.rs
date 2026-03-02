// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 7)
// ==============================

// Topic: Trait Bound Syntax (<T: Trait>)
// Feel:
// "Bhai, 'impl Trait' aur '<T: Trait>' mein 'Aasman-Zameen' ka farq tab aata hai jab hume
// parameters ko EK HI type ka rakhna ho."

trait Accommodation {
    fn info(&self) -> String;
}

struct Hotel {
    name: String,
}
impl Accommodation for Hotel {
    fn info(&self) -> String {
        format!("🏨 Hotel: {}", self.name)
    }
}

struct AirBnB {
    host: String,
}
impl Accommodation for AirBnB {
    fn info(&self) -> String {
        format!("🏠 AirBnB: {}", self.host)
    }
}

// ------------------------------
// CASE 1: 'impl Trait' (Khichdi allowed)
// ------------------------------
// Feel: "Bhai, mujhe do cheezein do, bas wo Accommodation honi chahiye.
// Pehla Hotel aur doosra AirBnB ho sakta hai? YES!"
fn mix_allowed(p1: &impl Accommodation, p2: &impl Accommodation) {
    println!("Mix allowed: {} AND {}", p1.info(), p2.info());
}

// ------------------------------
// CASE 2: Trait Bound '<T>' (Same Type Only)
// ------------------------------
// Feel: "T ek placeholder hai. Jab ek bar T decide ho gaya (e.g. Hotel),
// toh doosra parameter bhi 'Hotel' hi hona padega! Khichdi NOT allowed."
fn same_type_only<T: Accommodation>(p1: &T, p2: &T) {
    println!("Same type forced: {} AND {}", p1.info(), p2.info());
}

fn main() {
    let taj = Hotel {
        name: "Taj".to_string(),
    };
    let hyatt = Hotel {
        name: "Hyatt".to_string(),
    };
    let flat = AirBnB {
        host: "Suresh".to_string(),
    };

    println!("--- 'impl Trait' Demo ---");
    // Yaha Hotel aur AirBnB mix ho sakte hain!
    mix_allowed(&taj, &flat);

    println!("\n--- Trait Bound '<T>' Demo ---");
    // Dono Hotel hain? Chalega!
    same_type_only(&taj, &hyatt);

    /*
    🛑 FEEL WALA ERROR:
    Agar main same_type_only(&taj, &flat) likhu toh Error aayega!

    Kyun?
    Compiler bolega: "Bhai, pehle tune 'T' ko 'Hotel' bol diya (p1),
    toh ab 'p2' bhi 'Hotel' hona chahiye, par tune 'AirBnB' de diya!"
    */
}

/*
EXPECTED OUTPUT:
--- 'impl Trait' Demo ---
Mix allowed: 🏨 Hotel: Taj AND 🏠 AirBnB: Suresh

--- Trait Bound '<T>' Demo ---
Same type forced: 🏨 Hotel: Taj AND 🏨 Hotel: Hyatt
*/

/*
🧠 Deep Feel:
1. `impl Trait` is "Anonymous": Har parameter apna alag type ho sakta hai.
2. `<T: Trait>` is a "Named Type": Ek baar T define ho gaya, toh poore function mein jaha-jaha T hai,
waheen concrete type (Hotel) honi chahiye.

Moral: Jab aapko 'Homogeneous' (Same types) chahiye, toh `<T>` use karo.
Jab 'Heterogeneous' (Mixing allowed) chahiye, toh `impl Trait` use karo.
*/
