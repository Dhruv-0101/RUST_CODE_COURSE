// ==============================
// RUST CLOSURES — FULL FEEL VERSION (EPISODE 7)
// ==============================

// Topic: The Closure Traits (Fn, FnMut, FnOnce)
// Feel:
// "Bhai, jab ek function kisi closure ko 'Hire' karta hai,
// toh wo pehle 'Contract' check karta hai:
// 'Kya tum sirf dekhoge (Fn), ya badloge (FnMut), ya khaa jaoge (FnOnce)?'"

// ------------------------------
// 1. THE REPETITIVE CONTRACT (FnMut)
// ------------------------------
// Feel: `execute_thrice` kehta hai: "Main tumhe 3 baar call karunga."
// Isliye `F` ko `FnMut()` hona padega (taaki wo cheezon ko badal sake multiple times).
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(), // Shart: "Bhai, badal sakte ho par 3 bar chalna hai."
{
    println!("--- Manager is triggering the procedure ---");
    procedure();
    procedure();
    procedure();
}

fn main() {
    let mut bosses = vec!["Boris"];

    // Closure: Ye 'bosses' mein naya boss add karta hai.
    // Isse Mutable access chahiye, isliye ye `FnMut`.
    let closure = || {
        println!("Robot adding a boss...");
        bosses.push("Alexandra");
    };

    // Procedure manager ko closure de diya
    execute_thrice(closure);

    println!("Total Bosses: {:?}", bosses);
}

/*
EXPECTED OUTPUT:
--- Manager is triggering the procedure ---
Robot adding a boss...
Robot adding a boss...
Robot adding a boss...
Total Bosses: ["Boris", "Alexandra", "Alexandra", "Alexandra"]
*/

/*
🧠 Deep Feel:
1. Fn: Sabse shareef. Sirf variables ko reads karta hai. Isse kitni bhi bar use karo.
2. FnMut: Worker. Bahar ke variables ko change karta hai. Isse bhi kitni bhi bar use karo.
3. FnOnce: One-timer. Variables ko move/consume kar leta hai. Ek hi bar chalta hai.

Relationship Hierarchy:
Fn ⊂ FnMut ⊂ FnOnce
(Matlab jo closure Fn hai, wo FnMut aur FnOnce bhi implement karta hai automatically!)

Bhai, ye 'HR Management' hai mini robots ke liye.
*/
