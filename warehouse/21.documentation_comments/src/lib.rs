///tools for managing a inventory
pub mod inventory;
///tools for managing orders
pub mod orders;

/// re-exporting items from the inventory module
pub use inventory::products::{Item, ProductCategory};
/// re-exporting floor space and manager from the inventory module
pub use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
/// re-exporting manager from the orders module
pub use orders::MANAGER as ORDERS_MANAGER;
