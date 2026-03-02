// ==============================
// TRAITS IN RUST — FULL FEEL VERSION (EPISODE 25)
// ==============================

// Topic: Formatter Methods (debug_struct, debug_list, etc.)
// Feel: 
// "Bhai, manually write! karna mehnat ka kaam hai. 
// Rust hume Formatter ke andar bane-banaye tareeke (methods) deta hai 
// taaki hum apne data ko standard Struct ya List ki tarah dikha sakein bina manual braces {} lagaye."

use std::fmt::{Debug, Formatter, Result};

struct Classroom {
    name: String,
    students: Vec<String>,
}

impl Debug for Classroom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Hum built-in helper methods use karenge
        f.debug_struct("🏫 SchoolClass") // Struct ka naam
            .field("Section", &self.name) // Field 1
            .field("StudentList", &self.students) // Field 2 (Internally students ka debug call hoga)
            .finish() // Final result build karo
    }
}

fn main() {
    let my_class = Classroom {
        name: String::from("10-A"),
        students: vec!["Aryan".to_string(), "Isha".to_string(), "Zaid".to_string()],
    };

    println!("--- Formatter Magic ---");
    println!("{:#?}", my_class); // # used for pretty print
}

/*
EXPECTED OUTPUT:
--- Formatter Magic ---
🏫 SchoolClass {
    Section: "10-A",
    StudentList: [
        "Aryan",
        "Isha",
        "Zaid",
    ],
}
*/

/*
🧠 Deep Feel:
Formatter methods (debug_struct, debug_tuple, debug_list, debug_map, debug_set) 
hume allow karte hain ki hum custom types ko standard look-and-feel dain.
Isse debug logs consistent rehte hain aur hume manual brackets aur commas ki chinta nahi karni padti.
*/