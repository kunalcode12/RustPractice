
pub const FLOOR_SPACE:i32 = 10000;

//below we are writing pub as by default every element in this block is private,so if we want to access something from this module we have to use pub in front of the element we want to use outside
pub const MANAGER:&str = "Ivan Inventory";

//The crate Prefix

pub fn talk_to_manager() {
    println!("Hey, {MANAGER}, how's your coffee? What do you think of {:?}",
    ProductCategories::Ladder
    )
    //alternetive way of above code by using crate
    // println!("Hey, {}, how's your coffee?",crate::inventory::MANAGER);
}


//shifted all these code in products module
// #[derive(Debug)]
// //Public Enums, Public Structs, and Public Fields
// pub enum ProductCategories {
//     Ladder,
//     Hammer,
// }


// //so in below code even if declare struct as pub ,it inside element is still private so in another module ,we only have access to struct name ,not its element ,so to use these elements we need to declare elements as private also
// #[derive(Debug)]
// pub struct Item {
//     pub name:String,
//     pub category:ProductCategories,
//     pub quantity:u32,
// }

//so below this module need to use pub ,to use it in main.rs even if the elements in it is already public ,as code in rust is by default private
pub mod products;

//Using pub use to Export Names from Submodules
//pub keyword in front of use ->its exports the 'Item' and 'ProductCategory' names directly from the 'inventoy' module
//so now we will be able to access 'Item' and 'ProductCategory' by accessing the 'inventory' module from the top level file and that top level file will no longer have to reach further down into 'products' 
pub use products::{Item,ProductCategories};


