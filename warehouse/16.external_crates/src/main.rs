mod inventory;
mod orders;

use fake::{Fake, Faker};

use inventory::products::{Item, ProductCategory};
use inventory::{FLOOR_SPACE, MANAGER};
use orders::MANAGER as ORDERS_MANAGER;


fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;

    println!("My favorite category of item is {:?}", favorite_category);

    // ---------------------------------------------------------
    // 🤖 THE ROBOT CHEF (Faker vs. New)
    // ---------------------------------------------------------
    // RASTA A: MANUAL (Aapne ingredients bataye)
    // let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);

    // RASTA B: AUTOMATED (Robot ne random data bhar diya)
    // Yahan 'new' use nahi hua kyunki 'Faker' ne 'Item' struct ki 
    // metadata (Dummy trait) ko padha aur apne aap ek object bana diya.
    let tall_ladder: Item = Faker.fake(); 
    
    // NOTE: Ye tabhi chalta hai jab struct par '#[derive(Dummy)]' laga ho!
    // ---------------------------------------------------------

    println!("{:?}", tall_ladder);
}
//cargo run