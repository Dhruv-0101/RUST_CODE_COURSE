// ==============================
// RUST LIFETIMES — FULL FEEL VERSION (EPISODE 14)
// ==============================

// Topic: Lifetimes in Structs (Storing Borrowed Values)
// Feel: 
// "Bhai, socho aap ek Box (Struct) bana rahe ho aur usme ek reference rakh رہے ho. 
// Rust darr jata hai! 'Agar box toh zinda hai, par andar rakha reference mar gaya, toh?'. 
// Is darr ko khatam karne ke liye hume Struct ko Lifetime tag `'a` dena padta hai."

#[derive(Debug)]
struct TrainSystem<'a> {
    // Feel: 'a means: "Ye name reference hamesha TrainSystem se zyada zinda rahega!"
    name: &'a str, 
}

fn main() {
    let name = String::from("AmTrak");
    
    // Struct ke andar borrow rakha
    let nj_transit = TrainSystem { name: &name };

    println!("Train System Name: {}", nj_transit.name);

    {
        let local_name = String::from("Metro");
        let local_train = TrainSystem { name: &local_name };
        println!("Inner: {:?}", local_train);
    } // <--- local_name mar gaya, toh local_train bhi INVALID ho gaya.

}

/*
EXPECTED OUTPUT:
Train System Name: AmTrak
Inner: TrainSystem { name: "Metro" }
*/

/*
🧠 Deep Feel:
Agar aap struct mein `&` use karte ho, toh `'a` likhna MANDATORY hai. 
Rust ko ye guarantee chahiye ki struct kabhi bhi ek 'Dangling Reference' ko 
hold nahi karega. 
Jab aap `struct Book<'a> { title: &'a str }` likhte ho, 
toh aap 'Constraint' laga rahe ho: 
"Bhai, ye Book tab tak valid hai jab tak uska title reference valid hai."
*/
