//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Option Enum
//The Option enum models a scenario where a type could be a valid or nothing at all.

//syntax->
//Option::None represents an absent value

//Option::Some(T) represents a present value

#[derive(Debug, Clone, Copy)]
//so after declaring ,clone and copy ,so whenever we will use below enum ,it will not transfer the ownership ,it will automatically impliment the copy trait
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    //below the ownership will not get transfered due to this -> #[derive(Debug, Clone, Copy)]
    fn unwrap(self) ->i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
            // Self::Some(value) => value
        }
    }

    fn unwrap_or(self, fallback_value: i32) ->i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn main() {
    let a=Option::Some(5);
    let b=Option::Some("hello");
    let c=Option::Some(true);

    //if we want to specify a valid type manually
    let d:Option<i16>=Option::Some(5);

    //we can also use turbofish syntax
    let e=Option::<i16>::Some(7);

    //if we are using none then we have to provide the type anotation
    let f:Option<&str>=Option::None;


    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The get Method on an Array
    let musical_instrument=[
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    //the type returning here is a option ,it not a string or &string,so in println() we have to use debug formatter
    let bass: Option<&String>=musical_instrument.get(2);
    println!("{:?}",bass);

    //so below we are trying to access the invalid index in the code but ,it will not show error as we are using get() method ,and that will return an option which can have invalid value and not show error
    let invalid_instrument=musical_instrument.get(100);
    println!("{:?}",invalid_instrument); //Output=None



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The unwrap and expect Methods
    //The unwarp method attempts to extract the associated data out of the some variant
    let valid_instrument=bass.unwrap();
    println!("{valid_instrument}");

    //the unwrap() will not work for the Option which have nome value
    // invalid_instrument.unwrap();

    //another very similar method to unwrap() is expect() it is basically identical to unwrap() but it allows us to customize the error message
    let valid_instrument1=bass.expect("Unable to retrive element");
    // invalid_instrument.expect("Unable to retrive element");




    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The match Keyword with Option Enum
    let bass1: Option<&String>=musical_instrument.get(2);

    // match bass1 {
    //     Option::Some(instrument) => println!("Playing the {instrument}"),
    //     Option::None => println!("Singing with my voice"),
    // }

    //function for above code
    //as rust impliments the copy trait on 'Option enum' so in below code there is no transfer of the ownership and we can use this 'bass1' variable even after the below function
    play(bass1);
    println!("{:?}",bass1);

    let invalid_instrument=musical_instrument.get(100);

    // match invalid_instrument {
    //     Option::Some(instrument) => println!("Playing the {instrument}"),
    //     Option::None => println!("Singing with my voice"),
    // }

    //function for above code
    play(invalid_instrument);



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Returning an Option Enum from a Function
    let availability = is_item_in_stock(true, true);
    println!("{:?}",availability);

    match availability {
        Option::Some(value) => println!("Item is available: {value}"),
        Option::None => println!("Your item doesn't exist in our system"),
    }




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Top-Level Option Variants
    //Rust Prelude -> It is a collection of named constructs that are available automatically in every program.
    match availability {
        //so below we dont have to write Option:: as Some and None are Top level variants,below is just a simplified form 
        Some(value) => println!("Item is available: {value}"),
        Option::None => println!("Your item doesn't exist in our system"),
    }


    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The unwrap_or Method
    // let present_value = Option::Some(14);
    // let missing_value = Option::None;

    //alternative of above codes
    let present_value = Some(14);
    let missing_value: Option<i32> = None;

    println!("{}",present_value.unwrap_or(0));

    //so in below code we have a fallback value so if the missing_value is not valid or None then it will not show error but will give output whatever is in _or() parenthesis
    //whatever is fallback value in _or() ,it should be of same type as the type mentioned for 'missing_value' type
    println!("{}",missing_value.unwrap_or(100));



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Building Option from Scratch
    let some_option = MyOption::Some(100);
    println!("{}",some_option.unwrap());

    // let none_option=MyOption::None;
    // println!("{}",none_option.unwrap());

    let some_option1 = MyOption::Some(100);
    println!("{}",some_option1.unwrap_or(14));

    let none_option=MyOption::None;
    println!("{}",none_option.unwrap_or(10));



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The Result Enum
    //This Enum models the outcome of an evaluation that can produce either a success or an error.
    //So the Option enum is for representing presence versus absence ,while Result is for modeling success vs error
    
    //->The Result enum models an evaluation that can produce either a success or an error
    //->The Ok varient indicates a success. it stores an associated piece of data of generic type T.
    //->The Err varient indicates an error.it stores an associated piece of data of generic type E.
    // pub enum Result<T,E> {
    //     Ok(T),
    //     Err(E),
    // }

    //as in below code we are not using the second paramter ,so be have to write the type manually
    let ok:Result<i32, &str> = Result::Ok(5);
    //as Result automatically impliment debug trait ,so we only manually have to impliment display trait
    println!("{ok:?}");

    let disaster:Result<i32, &str> = Result::Err("Something went wromg");
    println!("{disaster:?}");



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The parse Method on a string
    let text="50";

    //so to convert above 'text' to the equivalent integer
    //so below if the parsing is successful then Result enum will return 'Ok' varient and if not the 'Err' varient ,as parse use Result enum to impliment its work
    //so we will use turbofish operator ::<> to provide the type to Ok and Err
    let text_as_number = text.parse::<i32>();
    println!("{:?}",text_as_number);

    let text1="Alabama";
    let text_as_number1 = text1.parse::<i32>();
    println!("{:?}",text_as_number1);



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Returning a Result Enum from a Function
    let result=divide(10.0, 2.0);
    
    match result {
        Ok(calculation) => println!("Result:{}", calculation),
        Err(message) => println!("Error: {}", message),
    }



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Result Methods
    let result1=divide(10.0, 2.0);
    println!("{}",result1.unwrap());
    // println!("{}",result1.unwrap_or(0.0));




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Nuances of unwrap Method on Result
    let my_result = operation(true);

    //so below the ownership of my_result transfers to content
    let content= match my_result {
        Ok(message) => message,
        Err(error) => error,
    };
    println!("{}",my_result.unwrap());
}


fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}

fn is_item_in_stock(item_is_in_system:bool,item_is_in_stock:bool) ->Option<bool> {
    if item_is_in_system && item_is_in_stock {
        //so below is one of the only two ways we can return a value from this function as this function returns Option enum as we can see after (->) this sign above ,the two return options= Option::some() and Option::none
        // Option::Some(true)
        //alternative of above ,simplified version but works as same
        Some(true)
    } else if item_is_in_system {
        // Option::Some(false)
        //alternative of above ,simplified version but works as same
        Some(false)
    }else {
        // Option::None
        //alternative of above ,simplified version but works as same
        None
    }
}

fn divide(numerator:f64, denominator:f64) ->Result<f64 , String> {
    if denominator == 0.0 {
        // Result::Err("Cannot divide by zero".to_string())
        Err("Cannot divide by zero".to_string())
    }else {
       Ok( numerator / denominator)
    }
}

fn operation(great_success: bool) ->Result<&'static str,&'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}