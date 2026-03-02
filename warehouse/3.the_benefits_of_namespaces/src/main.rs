mod inventory {
    const FLOOR_SPACE: u32 = 10000;
    pub const MANAGER: &str = "ivan inventory";

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

mod orders {
    pub const MANAGER: &str = "susan orders"
}

fn main(){
    println!("The manager of the inventory is {}", inventory::MANAGER);
    println!("The manager of the orders is {}", orders::MANAGER);
}