///////////////////////////////////////////////////////////////////////////////
//Enum
//An enum is a type that represents a set of possible values.Each possible value is called a variant

// #[derive(Debug)]
// enum CardSuit {
//     Hearts,
//     Diamonds,
//     Spades,
//     Clubs,
// }

// struct Card {
//     rank:String,
//     suit:CardSuit,
// }

// fn main() {
//     let first_card:CardSuit=CardSuit::Hearts;
//     let mut  second_card=CardSuit::Spades;

//     //this assigning some new data/value will only work if we assign the value from same enum only ,it will not work for like strings,float,integer, and all
//     second_card=CardSuit::Clubs;
//     println!("{:?}",second_card);

//     let card_suits=[CardSuit::Hearts,CardSuit::Clubs];
//     let card_suits1=(CardSuit::Hearts,CardSuit::Spades);

// }


//////////////////////////////////////////////////////////////////////////////////////
//Enum with Associated Values I

use std::iter::Cycle;

#[derive(Debug)]
enum PaymentMethodType {
    //below this is tuple varient because its a variant that holds an associated tuple of data
    //there are different types of variant we can store within an enum
    //so below is a enum with a associates data type that will live with that variant
    CreditCard(String),
    DebitCard(String),
    PayPal(Credentials),
}

#[derive(Debug)]
enum PaymentMethodType1 {
    CreditCard(String),
    DebitCard(String),
    //so here we directly define as struct
    PayPal {
        username:String,
        password:String,
    }
}


#[derive(Debug)]
struct Credentials {
    username:String,
    password:String,
}

//Nesting Enums in Enums

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurentItem {
    Burrito {meat:Meat, beans:Beans},
    Bowl {meat:Meat, beans:Beans},
    VeganPlate,
}

enum OperatingSystem {
    Windows,
    MacOS,
    Linux
}

enum LaunderyCycle {
    Cold,
    Hot {temperature: u32},
    Delicate (String),
}

//Defining Methods on Enums
impl LaunderyCycle {
    fn wash_laundery1(&self) {
        match self {
            LaunderyCycle::Cold => {
                println!("Running the laundery with cold temperature")
            },
            LaunderyCycle::Hot { temperature } => {
                println!("Running the laundery with a temperature of {temperature}");
            },
            LaunderyCycle::Delicate(fabric_type) => {
                println!("Running the laundery with a delicate cycle for {fabric_type}");
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("Your item is being preped for shipment");
            // }

            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered");
            }

            other_status => {
                println!("Your item is {other_status:?}");
            }

            // _ => {
            //     println!("Your item is not there yet");
            // }
        }
    }
}


enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favorite");
            }

            //this below match will match with any value given in percent so that's why we dont have to use _ this method
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version");
            }

            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

//The if let Construct

enum Milk1 {
    Lowfat(i32),
    Whole,
    NonDairy {kind: String},
}



fn main() {
    // let visa=(
    //     PaymentMethodType::CreditCard,
    //     String::from("0034-5678-9012-3456")
    // );

    //better approach then above
    let visa=PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let mastercard=PaymentMethodType::DebitCard(String::from("2532-1295-8735-2345"));
    println!("{:?}",visa);
    println!("{:?}",mastercard);



    ///////////////////////////////////////////////////////////////////////////////////////////////////////
    //Enum with Associated Values II
    //different enum variants can store different data,they can have different amount of data of different types
    let mut my_payment_method=PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    // my_payment_method=PaymentMethodType::PayPal(String::from("bob@gmail.com"), String::from("password"));

    println!("{:?}",my_payment_method);



    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Struct Variants
    //A Struct variant stores associated data in fields rather than by posistion.Each piece of data has an associated name.
    let paypal_credentials=Credentials {
        username:String::from("bob@gmail.com"),
        password:String::from("password"),
    };

    let paypal=PaymentMethodType::PayPal(paypal_credentials);
    println!("{:#?}",paypal);

    //now alternative and better method then above
    let paypal=PaymentMethodType1::PayPal { 
        username: String::from("bob@gmail.com"), 
        password: String::from("password"),
    };
    println!("{:#?}",paypal);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Nesting Enums in Enums
    let lunch=RestaurentItem::Burrito{
        meat:Meat::Steak,
        beans:Beans::Pinto
    };
    let dinner=RestaurentItem::Bowl { 
        meat: Meat::Chicken, 
        beans: Beans::Black
    };
    let abandoned_meal=RestaurentItem::VeganPlate;
    println!("Lunch was {lunch:?} and dinner was {dinner:?}");
    println!("Nobody ate {abandoned_meal:?}");



    ///////////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword I
    let number=20;

    match number {
        5 => println!("The number is 5"),
        8=>println!("It's *!!"),
        _ => println!("It's something else",)
    }

    let my_computer=OperatingSystem::MacOS;
    let age= years_since_release(my_computer);
    println!("My computers operating system is {age} years old");



    ////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword II
    let dads_computer=OperatingSystem::Windows;
    let age=years_since_release1(dads_computer);
    println!("My dad's computer is {age} years old");



    ///////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword III
    wash_laundery(LaunderyCycle::Cold);
    wash_laundery(LaunderyCycle::Hot { temperature: 100 });
    wash_laundery(LaunderyCycle::Delicate(String::from("Silk")));



    ////////////////////////////////////////////////////////////////////////////////////////////////////
    //Defining Methods on Enums
    LaunderyCycle::Cold.wash_laundery1();
    let hot_cycle=LaunderyCycle::Hot { temperature: 100 };
    hot_cycle.wash_laundery1();

    let delicate_cycle=LaunderyCycle::Delicate(String::from("Silk"));
    delicate_cycle.wash_laundery1();  



    //////////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword IV - Catching Multiple Values
    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Ordered.check();



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword V - match with Exact Value
    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink(); //this is match with exact value
    Milk::Whole.drink();



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The if let Construct
    let my_beverage=Milk1::Whole;

    //below code is saying if my_beverage is the Whole varient(::Whole) of the 'Milk1' enum,then this block {} will execute otherwise nothing happens
    if let Milk1::Whole = my_beverage {
        println!("You have whole milk");
    }

    let my_beverage1=Milk1::Lowfat(2);

    if let Milk1::Lowfat(percent) = my_beverage1 {
        println!("Your beverage is {percent}% milk");
    }

    let my_beverage2=Milk1::NonDairy { kind: String::from("Oat") };

    if let Milk1::NonDairy { kind } = my_beverage2 {
        println!("Your beverage is {kind} milk");
    } else {
        println!("You have some other milk variant");
    }



    //////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The let else Construct
    let my_beverage_value=Milk1::Lowfat(2);

    //so below else block will only execute if 'my_beverage_value' is not equal to 'Lowfat' variant(::Lowfat(percent)) 
    //and if that is equal to 'Lowfat' variant(::Lowfat(percent)) then we will be able to use Lowfat(percent) this percent data/value after the block {} 
    let Milk1::Lowfat(percent) = my_beverage_value else {
        println!("You do not have the lowfat milk");
        return;
    };

    println!("{percent}% milk is available here");

}

fn years_since_release(os:OperatingSystem)->u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn years_since_release1(os:OperatingSystem)->u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        },
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}

fn wash_laundery(cycle: LaunderyCycle) {
    match cycle {
        LaunderyCycle::Cold => {
            println!("Running the laundery with cold temperature")
        },
        LaunderyCycle::Hot { temperature } => {
            println!("Running the laundery with a temperature of {temperature}");
        },
        LaunderyCycle::Delicate(fabric_type) => {
            println!("Running the laundery with a delicate cycle for {fabric_type}");
        }
    }
}