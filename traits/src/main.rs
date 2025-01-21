////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Traits
//1.A trait is a contract that describes functionality that a type should hev.
//2.We use the word implement to describe when a type honors a traits requirements.
//3.A trait definition declares the method(s) that a type implementing that trait must have

//Traits seen till now
//->Display and Debug traits require a type to define methods for presenting itself as a string.
//->The Clone trait requires a type to define a clone method for creating a duplicate of itself

//Implementaions
//->once we have defined a trait ,we can impliment it on structs and enums.The type promises to honor the traita contract.
//->Multiple types can impliment the same trait.
//->A type can impliment multiple traits.

use std::collections::HashMap;


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Defining a Trait
// trait Accommodation {
//     //so below &self is an immutable reference to an instance of whatever type to be implimenting the Accommodation trait, and whatever type the 'get_description' method is invoked on.
//     //so below is just a contract,its a list of requirements for any type that is implimenting the Accommodation trait to honor
//     fn get_description(&self) ->String;
//     fn book(&mut self, name:&str, night:u32) ->();
// }

/////////////////////////
//Default Implementations
trait Accommodation {
    //so below &self is an immutable reference to an instance of whatever type to be implimenting the Accommodation trait, and whatever type the 'get_description' method is invoked on.
    //so below is just a contract,its a list of requirements for any type that is implimenting the Accommodation trait to honor
    // fn get_description(&self) ->String;

    //so below is the default implimentaion ,so in below codes in impls if we dont impliment this below function then this below output will be shown and if impliment then that impl block one will show output
    fn get_description(&self) ->String {
        String::from("A wonderful place to stay")
    }
    fn book(&mut self, name:&str, night:u32) ->();
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Implementing Trait for Struct I
#[derive(Debug)]
struct  Hotel {
    name:String,
    reservation:HashMap<String, u32>
}

impl Hotel {
    fn new(name:&str)->Self {
        Self {
            name:name.to_string(),
            reservation:HashMap::new()
        }
    }
    //Calling Trait Method from Another Method
    fn summarize(&self)->String {
        format!("{}: {}",self.name,self.get_description())
    }
}



impl Accommodation for Hotel {
    fn get_description(&self) ->String {
        format!("{} is the pinnacle of luxury",self.name)
    }

    fn book(&mut self, name:&str, night:u32) ->() {
        self.reservation.insert(name.to_string(), night);
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Implementing Trait for Struct II
#[derive(Debug)]
struct AirBnB {
    host:String,
    guests:Vec<(String,u32)>
}

impl AirBnB {
    fn new(host:&str)->Self {
        Self {
            host:host.to_string(),
            guests:vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) ->String {
        format!("Please enjoy {}'s apartment",self.host)
    }

    fn book(&mut self, name:&str, night:u32) ->() {
        self.guests.push((name.to_string(), night));
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Traits for Function Parameter Constraints

//below we are saying 'entity' can be any type as long as it impliments the Accommodation trait
fn book_for_one_night(entity: &impl Accommodation) {
    println!("{}",entity.get_description());
}

fn book_for_one_night1(entity: &mut impl Accommodation,guest:&str) {
    entity.book(guest, 1);
}

//Trait Bound Syntax
//below in <T: Accommodation> here we are adding a boundary ,a contraint that generic type T is not any type ,it has to be some type that impliments the Accommodation trait
fn book_for_one_night2<T: Accommodation>(entity: &mut T,guest:&str) {
    entity.book(guest, 1);
}

fn mix_and_match(first: &mut impl Accommodation,second:&mut impl Accommodation,guest:&str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

//alternative of above but if will work if the both paramter is of same type where we used T
//so we will use another generic type
// fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T,second:&mut U,guest:&str) {
//     first.book(guest, 1);
//     second.book(guest, 1);
// }

fn main() {
    let mut hotel =Hotel::new("The Luxe");
    println!("{}",hotel.get_description());
    hotel.book("Piers", 5);
    println!("{:#?}",hotel);

    let mut airbnb =AirBnB::new("Peter");
    println!("{}",airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{:#?}",airbnb);


    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Calling Trait Method from Another Method
    let mut hotel1 =Hotel::new("The Luxe");
    println!("{}",hotel1.summarize());
    hotel1.book("Piers", 5);
    println!("{:#?}",hotel1);




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Traits for Function Parameter Constraints

    let mut hotel2 =Hotel::new("The Luxe");
    book_for_one_night(&hotel2);
    book_for_one_night1(&mut hotel2, "piers");
    println!("{:#?}",hotel2);

    let mut airbnb2 =AirBnB::new("Peter");
    book_for_one_night(&airbnb2);
    book_for_one_night1(&mut airbnb2, "Amanda");
    println!("{:#?}",airbnb2);




    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Trait Bound Syntax
    //It uses a generic ,but it then limits the generic type to only types that impliment a given trait
    //A trait bound requires that a generic type implement a specific trait

    let mut hotel3 =Hotel::new("The Luxe");
    book_for_one_night2(&mut hotel3, "piers");
    println!("{:#?}",hotel3);

    let mut airbnb3 =AirBnB::new("Peter");
    book_for_one_night2(&mut airbnb3, "Amanda");
    println!("{:#?}",airbnb3);

    mix_and_match(&mut hotel3, &mut  airbnb3, "Piers");
    println!("{hotel3:#?} {airbnb3:#?}");
}
