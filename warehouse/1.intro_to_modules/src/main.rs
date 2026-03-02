mod inventory {
    const FLOOR_SPACE: u32 = 10000;
    const MANAGER: &str = "ivan inventory";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manage() {
        println!("Hey, {},how's your coffeee?", MANAGER);
    }
}
fn main() {}
