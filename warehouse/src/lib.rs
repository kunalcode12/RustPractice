//////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Create Library Crate
pub mod inventory;

///Tools for order management
pub mod orders;

//so after using below code in the inventory.rs file as pub keyword in front of use ->its exports the 'Item' and 'ProductCategory' names directly from the 'inventoy' module
pub use inventory::{Item,ProductCategories,MANAGER as INVENTORY_MANAGER,FLOOR_SPACE};
pub use orders::MANAGER as ORDERS_MANAGER;