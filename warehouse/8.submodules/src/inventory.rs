pub const FLOOR_SPACE: u32 = 10000;
pub const MANAGER: &str = "ivan inventory";

pub mod products;
//inline
//inventory/products.rs
//inventory/products/mod.rs

pub fn talk_to_manager() {
    println!("Hey, {},how's your coffeee?", MANAGER);
}
