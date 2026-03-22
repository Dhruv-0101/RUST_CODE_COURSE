mod inventory;
mod orders;

// use inventory::products::{Item, ProductCategory};
// use inventory::products::*;

// use inventory::products;
// use inventory::products::ProductCategory

use inventory::products::{self, ProductCategory};

use inventory::{FLOOR_SPACE, MANAGER, talk_to_manager};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    talk_to_manager();

    let favorite_category = ProductCategory::Hammer;

    println!("My favorite category of item is {:?}", favorite_category);

    let tall_ladder = products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };

    println!("{:?}", tall_ladder);
}
