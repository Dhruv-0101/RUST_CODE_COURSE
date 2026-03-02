pub const FLOOR_SPACE: u32 = 10000;
pub const MANAGER: &str = "ivan inventory";

// pub use products::{Item, ProductCategory};


// #[derive(Debug)]
// pub enum ProductCategory {
//     Ladder,
//     Hammer,
// }

// #[derive(Debug)]
// pub struct Item {
//     pub name: String,
//     pub category: ProductCategory,
//     pub quantity: u32,
// }

pub mod products;
//  {
//     #[derive(Debug)]
//     pub enum ProductCategory {
//         Ladder,
//         Hammer,
//     }

//   #[derive(Debug)]
//     pub struct Item {
//         pub name: String,
//         pub category: ProductCategory,
//         pub quantity: u32,
//     }
// }

//for submodules
//inline
//inventory/products.rs
//inventory/products/mod.rs

pub fn talk_to_manager() {
    println!("Hey, {},how's your coffeee?", crate::inventory::MANAGER); //absolute
    println!("Hey, {},how's your coffeee?", MANAGER); //relative path
    println!("Hey, {:?},how's that?", products::ProductCategory::Ladder);
}
