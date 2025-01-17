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


//moved below code in different folder
// mod orders {
//     pub const MANAGER:&str = "Oliver Orderson";
// }

mod orders;

// use inventory::MANAGER;
// use inventory::products::ProductCategories;

//if i want to import more than one so,unlike the just above one ,which is just for importing one
use inventory::products::{Item,ProductCategories};
use  inventory::{talk_to_manager,FLOOR_SPACE,MANAGER};

//so in below code self refers to the products module,self refers to whatever is the last module before the scope resolutiion :: operator
use inventory::products::{self};

//alias ,we use as keyword to impliment alias
use inventory::MANAGER as INVENTORY_MANAGER;
use orders::MANAGER as ORDERS_MANAGER;

//so after using below code in the inventory.rs file as pub keyword in front of use ->its exports the 'Item' and 'ProductCategory' names directly from the 'inventoy' module
// pub use products::{Item,ProductCategories};
//we can write this now here as now we can directly access that module codes

// use inventory::{Item,ProductCategories};


//extarnal modules
//so below Fake is a trait and the Faker is a struct
use fake::{Fake,Faker};


//standard library
// use std::fmt;
// use std::io;

//alternative
use std::{
    fmt,
    io::{stdin,stdout}
};


//The Glob Operator
//sign -> *
use std::collections::*;

//lib.rs file
//so be can also put all these code we are importing below in lib.rs and use it here
// use warehouse::{Item,ProductCategories,FLOOR_SPACE,INVENTORY_MANAGER,ORDERS_MANAGER};

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
    println!("The Manager of our inventory is {}",inventory::MANAGER);



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Module as Folder
    println!("The Manager of our orders is {}",orders::MANAGER);



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Public Enums, Public Structs, and Public Fields
    println!("Our managers are {} and {} ,we have {} square feet of floor space", 
        inventory::MANAGER,
        orders::MANAGER, 
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    //below code will not work as we shifted some of the code from inventory file to different folder with same name of folder like inventory but the file name in that folder is products ,and be only have mod products; defined in inventory file
    // let favourite_category = inventory::ProductCategories::Hammer;
    // println!("My favourite category of item is {favourite_category:?}");

    // let tall_ladder = inventory::Item {
    //     name:String::from("Ladder-o-matic 2000"),
    //     category: favourite_category,
    //     quantity: 100,
    // };
    // println!("{:#?}",tall_ladder);



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Submodules
    //it is a module that lives within another ,underneath another module
    let favourite_category = inventory::products::ProductCategories::Hammer;
    println!("My favourite category of item is {favourite_category:?}");

    let tall_ladder = inventory::products::Item {
        name:String::from("Ladder-o-matic 2000"),
        category: favourite_category,
        quantity: 100,
    };
    println!("{:#?}",tall_ladder);




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The crate Prefix 
    //crate is like a absolute path ,and this crate start its path from main.rs
    println!("Our managers are {} and {} ,we have {} square feet of floor space", 
        crate::inventory::MANAGER,
        crate::orders::MANAGER, 
        crate::inventory::FLOOR_SPACE
    );



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The use Keyword I
    //the use keyword brings a name into the current scope.It creates a "shortcut" to a name in a nested module
    println!("Our managers are {} and {} ,we have {} square feet of floor space", 
        MANAGER,
        orders::MANAGER, 
        FLOOR_SPACE
    );

    talk_to_manager();

    let favourite_category = ProductCategories::Hammer;
    println!("My favourite category of item is {favourite_category:?}");
    let tall_ladder = Item {
        name:String::from("Ladder-o-matic 2000"),
        category: favourite_category,
        quantity: 100,
    };
    println!("{:#?}",tall_ladder);



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The self Keyword
    let favourite_category1 = ProductCategories::Hammer;
    println!("My favourite category of item is {favourite_category1:?}");

    let tall_ladder1 = products::Item {
        name:String::from("Ladder-o-matic 2000"),
        category: favourite_category1,
        quantity: 100,
    };
    println!("{:#?}",tall_ladder1);




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The super Keyword
    //so in order to get access to the parent ,we use special keyword called 'super' ,parent means the file or folder which this current code is in,like a directory above it and so to access the code inside that we use super
    //Super means above

    let favourite_category2 = ProductCategories::Hammer;
    println!("My favourite category of item is {favourite_category2:?}");

    //below is a constructor function ::new defined in products.rs file
    let tall_ladder2 =Item::new(
        String::from("Ladder-o-matic 2000"), 
        favourite_category2, 
        100
    );
        
    println!("{:#?}",tall_ladder2);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Create Aliases with the as Keyword
    //its like an alternative name

    println!("Our managers are {} and {} ,we have {} square feet of floor space", 
        INVENTORY_MANAGER,
        ORDERS_MANAGER, 
        FLOOR_SPACE
    );



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Using pub use to Export Names from Submodules




    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //External Crates
    //A dependency is an external library crate that we pull into our project.Our code depends on it to run
    println!("Our managers are {} and {} ,we have {} square feet of floor space", 
        INVENTORY_MANAGER,
        ORDERS_MANAGER, 
        FLOOR_SPACE
    );

    let fake_item:Item = Faker.fake();
    println!("{:?}",fake_item);

    let random_category:ProductCategories = Faker.fake();
    println!("{:?}",random_category);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The Standard Library
    //The standard library is a collection of modules built into Rust




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The Glob Operator
    //sign -> *



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Multiple Binary Crates
    //so we made a bin folder ,and whatever file in bin folder is treated as the binary executable same like main.rs ,so every file in those have a main function and can run indepedently
    



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Documentation Comments ->so below this (///) three slashes is the documantation comment

    /// Primary entry point into our warehouse program
    println!("This is our primary program");

}
