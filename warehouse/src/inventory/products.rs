//so below we are importing the third part module
//below dummy is a trait ,which is a contract that mandates the requirement that type has to live upto
use fake::Dummy;

//below in (Debug,Dummy) ,this will enable us to instantiate a dummy version of the item struct 
#[derive(Debug,Dummy)]
//Public Enums, Public Structs, and Public Fields
pub enum ProductCategories {
    Ladder,
    Hammer,
}


//so in below code even if declare struct as pub ,it inside element is still private so in another module ,we only have access to struct name ,not its element ,so to use these elements we need to declare elements as private also
#[derive(Debug,Dummy)]
pub struct Item {
    pub name:String,
    pub category:ProductCategories,
    pub quantity:u32,
}

//The super Keyword
//so functions and methods defined in the impl are private by default
//so in order to get access to the parent ,we use special keyword called 'super' ,parent means the file or folder which this current code is in,like a directory above it and so to access the code inside that we use super
//super means above
impl Item {
    pub fn new(name:String,category:ProductCategories,quantity:u32) ->Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}