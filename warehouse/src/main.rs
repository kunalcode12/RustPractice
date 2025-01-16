////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Crates
//->A package is a collection of crates
//->A crate is a collection of Rust code that produces an executable or a library
//->A crate is the smallest amount of code that teh Rust compiler considers at a time

//Types of Crates
/*
1.A binary crate is a crate that compiles to an executable.
    ->A binary crate has a main function that is the entrypoint for the executable

2.A library crate exports functionality for other Rust programs to share and use.
    ->A library crate does not have a main function and does not compile to be an executable program  
*/

//Intro to Modules
mod inventory;

mod orders {
    pub const MANAGER:&str = "Oliver Orderson";
}

fn main() {
    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The pub Keyword
    
    //so in below code MANAGER does not exist in this scope
    //MANAGER does not live at the top level of our program ,rather it is tucked away within the 'inventory' module
    // println!("The Manager of our inventory is {}",MANAGER);

    //so we will try the to do this by using scope resolution :: 
    println!("The Manager of our inventory is {}",inventory::MANAGER);
    println!("The Manager of our orders is {}",orders::MANAGER);



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Module as File
    
}
