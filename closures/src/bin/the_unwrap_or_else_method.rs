// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 9)
// ==============================

// Topic: The unwrap_or_else Method
// Feel:
// "Bhai, ye 'Backup Plan' hai.
// Ye kehta hai: 'Agar value hai toh wo de do,
// lekin agar None hai... toh hi is closure ko disturb karna!'"

fn main() {
    // ------------------------------
    // 1. OPTION IS SOME ✅
    // ------------------------------
    let option = Some("Salami");

    // Backup closure: Agar mangna pada toh hi "Pizza" dega.
    let closure = || {
        println!("Robot: Preparing backup pizza..."); // Ye print nahi hoga!
        "Pizza"
    };

    // Feel: Food already "Salami" hai, isliye closure kabhi chalega hi nahi.
    // Isse performance bachti hai!
    let food = option.unwrap_or_else(closure);
    println!("Main Dish: {}", food);

    // ------------------------------
    // 2. OPTION IS NONE ❌
    // ------------------------------
    let empty_box: Option<&str> = None;
    let pizza_fan = true;

    // Feel: Ab empty_box khali hai, toh backup plan ko trigger hona padega.
    let food = empty_box.unwrap_or_else(|| {
        println!("Robot: Box was empty, triggering backup logic...");
        if pizza_fan { "Pizza" } else { "Hot Pockets" }
    });

    println!("Fallback Dish: {}", food);
}

/*
EXPECTED OUTPUT:
Main Dish: Salami
Robot: Box was empty, triggering backup logic...
Fallback Dish: Pizza
*/

/*
🧠 Deep Feel:
1. Lazy Evaluation: `unwrap_or` (without else) mein aap 'Default Value' pehle hi calculate kar lete ho.
   Par `unwrap_or_else` mein value sirf TABHI calculate hoti hai jab zaroorat padti hai (Lazy).
2. Use Case: Agar default value calculate karna 'Mehnga' (High CPU/Time) hai,
   toh closure use karo taaki bewajah calculation na ho.

Bhai, ye ek 'Emergency Generator' hai jo tabhi chalta hai jab bijli jati hai.
*/
