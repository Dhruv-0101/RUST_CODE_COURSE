use intro_to_modules::{INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of space",
        INVENTORY_MANAGER,
        ORDERS_MANAGER,
        FLOOR_SPACE
    );
}