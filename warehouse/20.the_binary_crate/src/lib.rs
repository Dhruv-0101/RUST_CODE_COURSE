pub mod inventory;
pub mod orders;

pub use inventory::products::{Item, ProductCategory};
pub use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDERS_MANAGER;