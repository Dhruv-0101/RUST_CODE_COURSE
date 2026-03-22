use intro_to_modules::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

/// get a summary of the intro_to_modules
fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );
}
