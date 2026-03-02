mod inventory;
// {
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

//     fn talk_to_manage r() {
//         println!("Hey, {},how's your coffeee?", MANAGER);
//     }

//     // above code is inline

//     // in file---we have to make a file with his module name within the same folder

// }

mod orders;
// {
//     // pub const MANAGER: &str = "olive orderson";
//    // in folder---make a folder with name of module within the same folder and also in newly created folder make mod.rs

// }

// use inventory::MANAGER;
// use inventory::products::ProductCategory;
use inventory::products::{
    // self,
    Item,
    ProductCategory, 
};
use inventory::{
    talk_to_manager,
    FLOOR_SPACE,
    // MANAGER
};
// use orders::MANAGER;//error
// use orders::MANAGER as ORDERS_MANAGER;

// use inventory::{talk_to_manager, Item, ProductCategory, FLOOR_SPACE};

fn main() {
    // println!("the manager of our inventory is {}", inventory::MANAGER);
    // println!("the manager of our orders is {}", orders::MANAGER);

    println!(
        "our managers are {} and {}.and we have {} sq ft of floor space",
        orders::MANAGER,
        inventory::MANAGER,
        // MANAGER,
        // inventory::FLOOR_SPACE
        FLOOR_SPACE
    );

    inventory::talk_to_manager();

    // let favourite_category = inventory::products::ProductCategory::Hammer;
    let favourite_category = ProductCategory::Hammer;
    println!("our favourite category is {favourite_category:?}");

    println!("my favourite category of item is {favourite_category:?}");

    // let tall_Ladder = inventory::products::Item {
    // let tall_Ladder = Item {
    // let tall_Ladder = products::Item {
    //     name: String::from("tall Ladder"),
    //     category: favourite_category,
    //     quantity: 1,
    // };

    let tall_Ladder = Item::new(String::from("tall Ladder"), favourite_category, 1);
    println!("our favourite item is {tall_Ladder:?}")
}
