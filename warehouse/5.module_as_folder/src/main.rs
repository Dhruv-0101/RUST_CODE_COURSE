//agar inventory module ko alag file me rakhna hai to uske liye hum us file
// ka naam inventory.rs rakhte hai aur usme inventory module ke code ko likhte hai.
// phir us file ko main.rs me import karte hai using mod keyword. is tarah se hum apne
//  code ko organize kar sakte hai aur alag alag modules ko alag alag files me rakh sakte hai.

//agar ordermodule ko as folder me rakhna hai to uske liye hum ek folder banate hai jiska
// naam orders rakhte hai aur usme mod.rs file banate hai jisme order module ka code
// likhte hai. phir us folder ko main.rs me import karte hai using mod keyword.
// is tarah se hum apne code ko aur bhi zyada organize kar sakte hai aur alag alag
//  modules ko alag alag folders me rakh sakte hai.

mod inventory;

//inline module below
//  {
//     const FLOOR_SPACE: u32 = 10000;
//     pub const MANAGER: &str = "ivan inventory";

//     #[derive(Debug)]
//     enum ProductCategory {
//         Ladder,
//         Hammer,
//     }

//     #[derive(Debug)]
//     struct Item {
//         name: String,
//         category: ProductCategory,
//         quantity: u32,
//     }

//     fn talk_to_manage() {
//         println!("Hey, {},how's your coffeee?", MANAGER);
//     }
// }
mod orders;

// {
//     pub const MANAGER: &str = "susan orders";
// }

fn main() {
    println!("The manager of the inventory is {}", inventory::MANAGER);
    println!("The manager of the orders is {}", orders::MANAGER);
}
