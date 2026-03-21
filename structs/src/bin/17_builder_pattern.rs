#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

fn main() {
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(3);

    println!("Stats: {computer:#?}");
}
// #[derive(Debug)]
// struct Computer {
//     cpu: String,
//     memory: u32,
//     storage: u32,
// }

// // 🏗️ THE BUILDER: The "Chef" who takes your order
// struct ComputerBuilder {
//     cpu: String,
//     memory: u32,
//     storage: u32,
// }

// impl ComputerBuilder {
//     // 1. Initial configuration (Default settings)
//     fn new() -> Self {
//         Self {
//             cpu: String::from("Intel i3"), // Default CPU
//             memory: 8,                    // Default RAM
//             storage: 256,                  // Default Storage
//         }
//     }

//     // 2. Customization methods (Returns 'self' to allow chaining)
//     fn cpu(mut self, cpu: String) -> Self {
//         self.cpu = cpu;
//         self // Ownership moves back out!
//     }

//     fn memory(mut self, memory: u32) -> Self {
//         self.memory = memory;
//         self
//     }

//     fn storage(mut self, storage: u32) -> Self {
//         self.storage = storage;
//         self
//     }

//     // 3. The final step: Convert Builder into the real Computer
//     fn build(self) -> Computer {
//         Computer {
//             cpu: self.cpu,
//             memory: self.memory,
//             storage: self.storage,
//         }
//     }
// }

// fn main() {
//     // 🎭 THE FEEL: Chaining like a Sentence
//     let my_custom_pc = ComputerBuilder::new()
//         .cpu(String::from("M3 Max"))
//         .memory(64)
//         .storage(2000)
//         .build(); // <--- Handing over the finished product

//     println!("--- 🖥️ MY NEW COMPUTER ---");
//     println!("{:#?}", my_custom_pc);
// }

// // =========================================================
// // 🧠 BUILDER PATTERN: The "Feel" Guide
// // =========================================================

// /*
// 1. THE PROBLEM: "The Monster Constructor"
// -----------------------------------------
// Imagine if Computer had 20 fields.
// `Computer::new("M3", 64, 2000, "Liquid Cooling", "RGB", "Webcam"...)`
// It's impossible to remember the order! ❌

// 2. THE SOLUTION: The Builder Pattern
// -------------------------------------
// Instead of one giant function, we use small, named methods.
// It's like ordering a Pizza:
// - "Pizza please" (ComputerBuilder::new())
// - "Add Cheese" (.cpu())
// - "Thick Crust" (.memory())
// - "Bake it!" (.build())

// 3. WHY RETURN 'self'? (Method Chaining)
// ---------------------------------------
// In Rust, by returning `self`:
// - We MOVE the builder from one method to the next.
// - This allows us to use the `.` operator one after another.
// - It feels like a single continuous "Build Process".

// 4. THE `build()` METHOD
// ------------------------
// This is the "Point of No Return".
// The Builder is consumed, and the real `Computer` is born.
// You can't go back and change the CPU once the computer is built!

// SUMMARY:
// - Step-by-step construction.
// - Named parameters (no more confusing 'new' arguments).
// - Final `build()` converts the 'Order' into a 'Product'.
// */
// =========================================================
// ⚔️ THE BATTLE OF CHAINING: `&mut Self` vs `Self`
// =========================================================

/*
Aapne bahut sahi cheez observe ki! Dono hi chaining allow karte hain,
lekin unke kaam karne ka tarika bilkul alag hai.

--- 1. Aapka Version: `&mut Self` (The Borrowing Style) ---
fn upgrade_cpu(&mut self, ...) -> &mut Self { ... }

- FEEL: "Main ek existing computer ko upgrade kar raha hoon."
- MEMORY: Ye computer ko 'borrow' karta hai, use change karta hai,
  aur wahi reference wapas kar deta hai.
- USAGE: Aapko `let mut computer = ...` pehle likhna padega.
- PROS: Computer wahi rehta hai, memory address change nahi hota.

--- 2. Mera Version: `Self` (The Ownership Style) ---
fn cpu(mut self, ...) -> Self { ... }

- FEEL: "Main computer ko dismantle karke naya naya assemble kar raha hoon."
- MEMORY: Ye computer ko 'Consume' (Move) kar leta hai, use change karke
  ek 'Naya' computer wapas deta hai.
- USAGE: Builders mein ye standard hai kyunki hum aksar intermediate steps
  ko save nahi karna chahte (`computer = computer.cpu(...)`).
- PROS: Ye sabse "Rust-y" tarika hai chaining ka. Builder ke case mein,
  hume build() ke baad purana builder nahi chahiye hota, isliye Move
  (Self) best hai.

--- Kab Kaunsa Use Karein? ---
- Use `&mut Self`: Jab object pehle se ban chuka hai aur aap use sirf
  modify/upgrade karna chahte hain (jaise aapka upgrade system).
- Use `Self`: Jab aap ek naya object bana rahe hain (Builder Pattern)
  aur aap chahte hain ki `build()` call hone par purana builder delete
  ho jaye.
*/

// ⚖️ Kab kaunsa use karein?
// Feature	  &mut Self (Aapka)	                                      Self (Mera)
// Purpose	Kisi cheez ko Update karne ke liye.	                   Kisi cheez ko Build karne ke liye.
// Safety	   Original variable zinda (Alive) rehta hai.	            build() call hote hi builder "mar" (Move) jata hai.
// Ease	    Standard mutability rules follow karta hai.	            Most "Rust-y" way for Builder pattern.
