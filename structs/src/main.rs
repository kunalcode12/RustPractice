///////////////////////////////////////////////////////////////////////
//Struct
//A Struct is a container for related pieces of data.

//Rust has 3 kinds of structs:
//->Named Field Structs
//->Tuple-Like Structs
//->Unit-Like Structs

#[derive(Debug)]

//Now its scope is in the whole file
struct Coffee1 {
    price: f64,
    name: String,
    is_hot: bool,
}

#[derive(Debug)]
struct LanaDelRaySong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

//in below code ,in impl we are going to derive the methods for the LanaDelRaySong struct
impl LanaDelRaySong {
    //so now the first parameter in a struct method is always going to be the keyword "self" ,now what self is going to be can defer, but what it ultimately represent is the struct in some form.
    //we're going to invoke this below method on a 'LanaDelRaySong' instance,and Rust will actually pass in something to represent this 'self' parameter,but what that something is can vary,it is like when we saw a four diffrent ways that we could pass a struct into a function 
    //so here below 'Self' this bigger Self is the another way to represent like (self:LanaDelRaySong)
    //leaving only self will also work same as above statement ,as it will point to impl LanaDelRaySong  ,whatever is after impl ,that is what self or Self follow

    //so those 4 combinations
    //1.Immutable struct value (self parameter takes ownership)
    fn display_song_info(self) {
        println!("Title: {}",self.title);
        println!("Release Year:{}",self.release_year);
        println!("Duration:{}",self.duration_secs);
    }

    //2.Mutable struct value (self paramter takes ownership ,has permission to mutate)
     fn double_length(mut self) {
        self.duration_secs= self.duration_secs * 2;
        println!("{:#?}",self);
     }   

    //3.Immuatble reference to the struct instance (no ownership moved)

    //a->first method
    // fn display_song_info1(self:&LanaDelRaySong) {
        
    // }

    //b->second method
    // fn display_song_info1(self:&Self) {
        
    // }

    //c->Best method
    fn display_song_info1(&self) {
        println!("Title: {}",self.title);
        println!("Years since Release:{}",self.years_since_release());
        println!("Duration:{}",self.duration_secs);
    }

    //4.Mutable reference to the struct instance (no ownership moves,has permission to mutate)
    fn double_length1(&mut self) {
        self.duration_secs= self.duration_secs * 2;
        println!("{:#?}",self);
    }  


    ///////////////////////////////////////////////////////////
    //Methods with Multiple Parameters

    //so below i want to call 'is_longer_then' on one struct and then pass in another struct as the argument to compare the orignal struct with
    //second parameter is the another struct as the argument
    fn is_longer_than(&self,other:&Self)->bool {
        self.duration_secs>other.duration_secs
    }


    //////////////////////////////////////////////////////////////////
    //Calling Methods from Other Methods
    fn years_since_release(&self)->u32 {
        2025-self.release_year
    }


    ///////////////////////////////////////////////////////////////////
    //Associated Functions
    //below declaring 'new' associated function that going to contruct a new 'LanaDelRaySong' instance
    //so here in impl we mainly describe method ,so how does rust know that it is also not a method but function, so the reason is we dont write the parameter self as we were writing in methods

    // fn new(title:String,release_year:u32,duration_secs:u32)->LanaDelRaySong {
    //     LanaDelRaySong {
    //         title,
    //         release_year,
    //         duration_secs,
    //     }
    // }

    //Alternative of above
    // fn new(title:String,release_year:u32,duration_secs:u32)->Self {
    //     Self {
    //         title,
    //         release_year,
    //         duration_secs,
    //     }
    // }

    //now commenting above code to impliment same in just in other impl

}

//Multiple impl Blocks
impl LanaDelRaySong {
     ///////////////////////////////////////////////////////////////////
    //Associated Functions
    //below declaring 'new' associated function that going to contruct a new 'LanaDelRaySong' instance
    //so here in impl we mainly describe method ,so how does rust know that it is also not a method but function, so the reason is we dont write the parameter self as we were writing in methods

    // fn new(title:String,release_year:u32,duration_secs:u32)->LanaDelRaySong {
    //     LanaDelRaySong {
    //         title,
    //         release_year,
    //         duration_secs,
    //     }
    // }

    //Alternative of above
    fn new(title:String,release_year:u32,duration_secs:u32)->Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }

}

//////////////////////////////////////////////////////////////////////////////////////////////////
//Builder Pattern

#[derive(Debug)]
struct Computer {
    cpu:String,
    memory:u32,
    hard_drive_capacity:u32,
}

impl Computer {

    //below in return function in (->Self) it is same as (->Computer)
    fn new(cpu:String,memory:u32,hard_drive_capacity:u32)->Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    //so to impliment that chaining we return a self in the methods
    fn upgrade_cpu(&mut self,new_cpu:String)->&mut Self {
        self.cpu=new_cpu;
        self
    }

    fn upgrade_memory(&mut self,new_memory:u32)->&mut Self {
        self.memory=new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self,new_hard_drive_capacity:u32)->&mut Self {
        self.hard_drive_capacity=new_hard_drive_capacity;
        self
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////
//Tuple Structs
struct ShortDuration(u32,u32); //representing hours and minutes
struct  LongDuration(u32,u32);//Years and Months


///////////////////////////////////////////////////////////////////////////////////////////////////////////
//Practice

#[derive(Debug)]
struct Flight {
    origin:String,
    destination:String,
    price:f64,
    passenger:u32
}

impl Flight {
    fn new(origin:String,destination:String,price:f64,passenger:u32)->Self {
        Self {
            origin,
            destination,
            price,passenger
        }
    }

    fn change_destination(&mut self,new_destination:String) {
        self.destination=new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.20;
    }

    fn itenerary(&self) {
        println!("{} -> {}",self.origin,self.destination);
    }
}



fn main() {
    
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha=Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    //access structs field
    println!("My {} this morning cost {}.It is {} that it was hot",mocha.name,mocha.price,mocha.is_hot);

    //so here ownership of the string from the 'name' field inside the struct above ,mover from the 'name' field to the new owner which is the 'favorite_coffee' varaible
    //but this above rule does not apply to like 'price','is_hot' from the above struct because they are stored on stack,and if we assign that to some variable that will be copy of that variable
    let favourite_coffee=mocha.name;
    println!("{}",favourite_coffee);



    //////////////////////////////////////////////////////////////////
    //Overwrite Struct Fields

    let mut beverage=Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,
    };

    beverage.name=String::from("Caramel Macchiato");
    beverage.price=6.99;
    beverage.is_hot=true;

    println!("My {} this morning cost {}.It is {} that it was hot",beverage.name,beverage.price,beverage.is_hot);


    //////////////////////////////////////////////////////////
    //Create Structs in a Function
    let coffee: Coffee1= make_coffee(String::from("Latte"), 4.99, true);
    println!("My {} this morning cost {}.It is {} that it was hot",coffee.name,coffee.price,coffee.is_hot);


    //////////////////////////////////////////////////////////////////
    //struct update syntax
    let mocha:Coffee1=make_coffee(String::from("Mocha"), 4.99, true);

    //this below struct is totally independent of mocha struct above even if we are copying the struct from mocha and if we change the struct of mocha it will not effect the below struct 
    let caramel_macchiato=Coffee1 {
        name:String::from("Caramel Macchiato"),

        //so in below code the rest of the mocha struct value will be used here ,and also only those field which are not already mentioned here will come here 
        ..mocha
    };

    //below this {..mocha} will just copy the same struct as mocha here 
    //But one problem is here as mocha have a name:String type so the ownership of that name will be transfered to below variable ,so after below code we will not be able to get mocha.name
    let vanilla_macchito=Coffee1 {..mocha};
    println!("{}",vanilla_macchito.name);

    //below will not work as mentioned above
    // println!("{}",mocha.name);

    //so to solve above problem we can in vanilla_macchito can impliment name field like ->name:mocha.name.clone() 






    ////////////////////////////////////////////////////////////////////
    //Passing Structs into a Function

    let mut mocha1:Coffee1=make_coffee(String::from("Mocha"), 4.99, true);

    //here in below code the ownership of mocha1 is passed to below function parameter so after function the mocha does not exist means we cant access it after the below function
    // drink_coffee(mocha1);

    //now passing reference in the parameter
    drink_coffee1(&mocha1);

    /////////////////////
    //Mutable reference
    drink_coffee2(&mut mocha1);
    println!("{}",mocha1.price);



    ////////////////////////////////////////////////////////////////
    //Deriving Debug Trait for Struct
    let mocha2:Coffee1=make_coffee(String::from("Mocha"), 4.99, true);

    //below as struct also dont follow the display and debug trait so it will show error
    // println!("{}",mocha2);
    // println!("{:?}",mocha2);
    // println!("{:#?}",mocha2);

    //there is some way to derive the debug trait representation for any one of our structs
    //so we can use attribute,so we can use attribute to issue a command to the compiler,and that is to derive debug trait implimentation for struct
    //->So i implimented on the top of this file ->#[derive(Debug)] ,see on top of file
    //so now below code will work after i implimented just above code
    println!("{:?}",mocha2);
    println!("{:#?}",mocha2);



    ////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////
    //Defining Struct Methods

    let mut song:LanaDelRaySong=LanaDelRaySong { 
        title: String::from("SummerTime Sadness"), 
        release_year: 2022, 
        duration_secs: 231 
    };

    //now invoking the method on a type
    //in the below function invocation 'self' in the on top of this file in method perameter of LanaDelRaySong is going follow the first rule see rules on top
    //and also as the ownership is also passed to this below method parameter which is 'self' ,so after this below code we cant access ,song varaible which we defined just above

    // song.display_song_info();
    //we have to comment out the just above code as the above function runs the ownership will go to above function parameter so we cant get access be able to access the song variable after that


    //////////////////////////////////////////////////////////////////////////////////////
    //self Parameter as Mutable Struct Instance

    // song.double_length();
    //we have to comment out the just above code as the above function runs the ownership will go to above function parameter so we cant get access be able to access the song variable after that


    ///////////////////////////////////////////////////////////////////////////////////////
    //self Parameter as Immutable and Mutable References to Struct Instance
    song.display_song_info1();
    song.double_length1();
    song.display_song_info1();



    ////////////////////////////////////////////////////////////////////////////////////////
    //Methods with Multiple Parameters
    let blank_space:LanaDelRaySong=LanaDelRaySong { 
        title: String::from("SummerTime Sadness"), 
        release_year: 2022, 
        duration_secs: 231 
    };

    let all_to_well:LanaDelRaySong=LanaDelRaySong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_to_well){
        println!("{} is longer than {}",blank_space.title,all_to_well.title);
    }else {
        println!("{} is shorter than or equal to {}",blank_space.title,all_to_well.title);
    }



    ////////////////////////////////////////////////////////////////////////////////////////
    //Calling Methods from Other Methods
    blank_space.display_song_info1();



    ////////////////////////////////////////////////////////////////////////////////////////
    //Associated Functions
    //These are functions that are attached to a type
    //these functions are not methods as they do not live on an instance of the type
    //eg->String::new() ,so here new() here is an example of a contructor function ,it is a function that returns a string instance or string value
    //so syntax ->Typename::new() ,so here new is associative function ,and then that new() ,is going to create a new instance of that type
    let blank_space1=LanaDelRaySong::new(String::from("Blank Space"), 2014,231 );
    blank_space1.display_song_info1();



    ///////////////////////////////////////////////////////////////////////////////////////////
    //Multiple impl Blocks
    //see explanation on top in impl block




    //////////////////////////////////////////////////////////////////////////////////////////////////
    //Builder Pattern
    //A design pattern is a recommended way to write or structure code to solve specific problems
    let mut computer=Computer::new(String::from("M3 Max"), 64, 2);

    //so here below we continuesly have to write computer. again and again to call the method 
    computer.upgrade_cpu(String::from("M4 Max"));
    computer.upgrade_memory(128);

    //so here below we continuesly have to write computer. again and again to call the method ,so we have a chaining method in the builder pattern which we can use
    //so to impliment that chaining we return a self in the methods which we defined in the impl above at top
    computer.upgrade_cpu(String::from("M4 Max"))
    .upgrade_memory(128)
    .upgrade_hard_drive_capacity(4);

    println!("Stats:{:#?}",computer);





    //////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Tuple Structs
    let work_shift=ShortDuration(8,0);
    println!("{} hours {} minutes",work_shift.0,work_shift.1);

    let era=LongDuration(5,3);
    println!("{} years {} months",era.0,era.1);

    // let work_shift1=(8,0);
    // let era1=(5,3);

    // //so we can see below function will work for both struct one and only normal tuple one
    // go_to_work(work_shift1);
    // go_to_work(era1);

    go_to_work1(work_shift);


    ////////////////////////////////////////////////////////////////////////////////////////////
    //Practice
    let mut flight:Flight=Flight::new(String::from("New Delhi"), String::from("New York"), 45000.89, 2);
    println!("{:#?}",flight);

    flight.change_destination(String::from("Washington DC"));
    println!("{:#?}",flight);

    flight.increase_price();
    println!("{:#?}",flight);

    flight.itenerary();

    let new_flight=Flight {
        origin:String::from("Washington DC"),
        destination:String::from("Taxes"),
        ..flight
    };

    println!("{:#?}",new_flight);

}

//from below function we are implicitly returning a 'Coffee' instance
fn make_coffee(name:String,price:f64,is_hot:bool)->Coffee1 {
    // Coffee1 {
    //     name:name,
    //     price:price,
    //     is_hot:is_hot
    // }

    //better method then above
    Coffee1 {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(mut coffee:Coffee1) {
    println!("Drinking my delicious {}",coffee.name);
    coffee.is_hot=false;
}

fn drink_coffee1(coffee:&Coffee1) {
    println!("Drinking my delicious {}",coffee.name);
}

fn drink_coffee2(coffee:&mut Coffee1) {
    println!("Drinking my delicious {}",coffee.name);
    coffee.is_hot=false;
    coffee.price=10.99;
}

fn go_to_work(length:(i32,i32)) {
    println!("Passing time {} hours {} minutes",length.0,length.1);
}

fn go_to_work1(length:ShortDuration) {
    println!("Passing time {} hours {} minutes",length.0,length.1);
}



///////////////////////////////////////////////////////////////////
//Practice
/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)
 
Derive a Debug trait implementation for the Flight struct.
 
Define a `new` constructor function that returns a new
instance of a Flight.
 
Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.
 
Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.
 
Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).
 
Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.
 
Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/