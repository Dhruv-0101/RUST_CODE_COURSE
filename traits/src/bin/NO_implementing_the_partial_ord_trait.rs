// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 31 - PartialOrd)
// ==============================

// Topic: PartialOrd (Comparison/Ordering)
// Feel: 
// "Bhai, Equality (==) toh ho gayi. Lekin 'Bada' (>) aur 'Chhota' (<) kaise dekhein? 
// 'PartialOrd' hume ye power deta hai. 
// Isme ek method hota hai `partial_cmp` jo batata hai ki cheez Greater hai, Less hai, ya Equal."

use std::cmp::Ordering;

#[derive(Debug, PartialEq)] // PartialOrd ke liye PartialEq hona zaroori hai
struct Job {
    salary: u32,
    commute_time: u32,
}

// ==============================
// IMPLEMENTING PARTIALORD
// ==============================
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Feel: "Hamari priority sirf Salary hai. Commute time bhale hi kitna bhi ho!"
        self.salary.partial_cmp(&other.salary)
    }
}

fn main() {
    let dev_job = Job { salary: 120000, commute_time: 60 };
    let clerk_job = Job { salary: 50000, commute_time: 10 };

    println!("--- Job Market Comparison ---");
    // Comparison operators (<, >, <=, >=) internally partial_cmp use karte hain
    println!("Is Dev Job > Clerk Job? {}", dev_job > clerk_job);
    println!("Is Dev Job < Clerk Job? {}", dev_job < clerk_job);

    let same_salary_job = Job { salary: 120000, commute_time: 5 };
    println!("Is Dev Job >= Same Salary Job? {}", dev_job >= same_salary_job);
}

/*
EXPECTED OUTPUT:
--- Job Market Comparison ---
Is Dev Job > Clerk Job? true
Is Dev Job < Clerk Job? false
Is Dev Job >= Same Salary Job? true
*/

/*
🧠 Deep Feel:
PartialOrd hume allow karta hai comparison operators use karne ke liye.
Inland story: Kuch types aise hote hain jinhe hum compare NAHI kar sakte 
(jaise floats mein NaN). Isiliye ye `Option<Ordering>` return karta hai, 
kyunki result `None` bhi ho sakta hai agar comparison impossible ho.
Agar aapka data humesha comparable hai, toh aap `Ord` (Total Order) implement kar sakte ho.
*/

