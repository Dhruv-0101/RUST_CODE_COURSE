// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 1)
// ==============================

// Topic: Introduction to Closures
// Feel:
// "Bhai, Closures matlab 'Anjaan Functions' (Anonymous Functions).
// Ye aise mini-robots hain jinhe aap variable mein store kar sakte ho.
// Sabse bade maza? Ye apne bahar ke mahaul (variables) ko 'Capture' karke yaad rakh sakte hain!"

fn main() {
    // Bahar ka ek variable
    let multiplier = 3;

    // ------------------------------
    // 1. CLOSURE DEFINITION 🛠️
    // ------------------------------
    // Feel: Pipes |...| ke andar parameters aate hain, aur curly braces ke andar logic.
    // multiply_by ek variable hai, par behave function ki tarah karega!
    let multiply_by = |value: i32| -> i32 {
        // Look! multiplier bahar ka hai, par closure usse 'dekh' aur 'use' kar pa raha hai.
        value * multiplier
    };

    // Use it like a function
    let result = multiply_by(5);
    println!("Multiply result: {}", result);

    // ------------------------------
    // 2. ANOTHER QUICK CLOSURE 🚀
    // ------------------------------
    let product = |a: i32, b: i32| -> i32 {
        println!("Robot calculating product...");
        a * b
    };

    println!("3 * 10 is: {}", product(3, 10));
    println!("5 * 8 is: {}", product(5, 8));
}

/*
EXPECTED OUTPUT:
Multiply result: 15
Robot calculating product...
3 * 10 is: 30
Robot calculating product...
5 * 8 is: 40
*/

/*
🧠 Deep Feel:
1. Syntax: `let variable_name = |params| { logic };`
2. Power: Ordinary functions apne scope se bahar ke variables nahi dekh sakte.
   Par Closures 'Environment' ko capture karte hain.
3. Type: Closures anonymous hote hain. Inka type compiler khud decide karta hai.

Bhai, socho closures ek aise 'Task' hain jo aapne variable mein pack karke rakh diye
aur jab man chaha, tab 'Call' kar liya!
*/
