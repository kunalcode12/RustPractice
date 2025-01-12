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
    fn display_song_info(self) {
        //so those 4 combinations
        //1.Immutable struct value (self parameter takes ownership)
            println!("Title: {}",self.title);
            println!("Release Year:{}",self.release_year);
            println!("Duration:{}",self.duration_secs);

        //2.Mutable struct value (self paramter takes ownership ,has permission to mutate)

        //3.Immuatble reference to the struct instance (no ownership moved)

        //4.Mutable reference to the struct instance (no ownership moves,has permission to mutate)

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

    let song:LanaDelRaySong=LanaDelRaySong { 
        title: String::from("SummerTime Sadness"), 
        release_year: 2022, 
        duration_secs: 231 
    };

    //now invoking the method on a type
    //in the below function invocation 'self' in the on top of this file in method perameter of LanaDelRaySong is going follow the first rule see rules on top
    //and also as the ownership is also passed to this below method parameter which is 'self' ,so after this below code we cant access ,song varaible which we defined just above
    song.display_song_info();


    //////////////////////////////////////////////////////////////////////////////////////
    //self Parameter as Mutable Struct Instance


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