use fake::Dummy;

/// enum for product categories
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// struct for items
#[derive(Debug, Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

/// creates a new item
impl Item {
    /// creates a new item function
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}
//cargo doc --no-deps --open
