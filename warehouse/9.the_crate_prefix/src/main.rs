mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        crate::inventory::MANAGER,
        crate::orders::MANAGER,
        crate::inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_category = crate::inventory::products::ProductCategory::Hammer;

    println!("My favorite category of item is {:?}", favorite_category);

    let tall_ladder = crate::inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };

    println!("{:?}", tall_ladder);
}
