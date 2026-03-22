use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat White", "Almond Milk");

    let value = coffee_pairings
        .get("Flat White")
        .copied()
        .unwrap_or("Unknown Milk");
    // .get() returns an Option<&&str>
    // .copied() converts Option<&&str> to Option<&str>
    // .unwrap_or() returns the value if Some, or the default value if None

    println!("{value}");
}
