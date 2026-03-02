pub const FLOOR_SPACE: u32 = 10000;
pub const MANAGER: &str = "ivan inventory";

#[derive(Debug)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

pub fn talk_to_manager() {
    println!("Hey, {},how's your coffeee?", MANAGER);
}
