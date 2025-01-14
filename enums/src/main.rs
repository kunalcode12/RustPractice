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

}