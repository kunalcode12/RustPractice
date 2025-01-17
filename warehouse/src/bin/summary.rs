//Multiple Binary Crates
//so we made a bin folder ,and whatever file in bin folder is treated as the binary executable same like main.rs ,so every file in those have a main function and can run indepedently
//we can use warehouse in any file as it is declared as whole project name in the cargo.toml 

use warehouse::{FLOOR_SPACE,INVENTORY_MANAGER,ORDERS_MANAGER};

/// Get a summary of our current managers
fn main() {
    println!("Our manager are {} and {} ,We have {} square feet of space", 
    INVENTORY_MANAGER,ORDERS_MANAGER,FLOOR_SPACE
    );
}