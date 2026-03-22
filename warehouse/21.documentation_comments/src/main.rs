// // mod inventory;
// // mod orders;

// use fake::{Fake, Faker};

// use intro_to_modules::inventory::products::{Item, ProductCategory};
// use intro_to_modules::inventory::{FLOOR_SPACE, MANAGER};
// use intro_to_modules::orders::MANAGER as ORDERS_MANAGER;

// // use inventory::products::{Item, ProductCategory};
// // use inventory::{FLOOR_SPACE, MANAGER};
// // use orders::MANAGER as ORDERS_MANAGER;

// fn main() {
//     println!(
//         "Our managers are {} and {}. We have {} square feet of floor space",
//         MANAGER, ORDERS_MANAGER, FLOOR_SPACE
//     );

//     let favorite_category = ProductCategory::Hammer;

//     println!("My favorite category of item is {:?}", favorite_category);

//     // let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
//     let tall_ladder: Item = Faker.fake();

//     println!("{:?}", tall_ladder);
// }
// //cargo run

///primary function of the program
fn main() {
    println!("Hello, world!");
}
