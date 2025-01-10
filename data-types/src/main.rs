////////////////////////////////////////////
//Data Types
//Every rust value has a data type.
//Rust is a statically types language ,which means compiler should know the types of all variables at complile time

//There are different kind of data types
//1.Scaler type->it is a type that holds a single value
//->Rust has 4 scaler types:integers,floating-point numbers,Booleans and characters.
//->Integer->whole number,Floating-point number->decimal number

// fn main() {
//     //below code will give error as -212 is outside the range of i8 memory
//     // let eight_bit: i8=-212;

//     //below code will also show error as u8 here ,in u is for unsigned so it start from 0 till the positive range of it ,so for negative value it will show error
//     // let eight_bit_unsigned:u8=-15;

//     let sixteen_bit_signed:i16=-32500;

//     //below we have a benifit for more memeory as it can go till 64000 + ,but the above also can store -ve number so ,its range is not much ,so above value will not be able to store 64000 like below 
//     let sixeen_bit_unsigned:u16=64000;

//     let thirty_two_bit_signed=-2147483648;
//     let thirt_two_bit_unsigned:u32=4271737778;

//     let some_value=67;

//     //we can also write the numbers with underscore
//     let sixteen_bit_signed:i16=32_500;

//     /////////////////////////////////////////////////
//     //unsize and isize types
//     //usize and isize are aliases for an existing integer type,so they are not diggerent or unique types
//     //they are like nickname for another existing rust type


//     //usize ->it is for unsigned value
//     //this means on a 64 bit computer it is of size 64 and on 32 bit computer it is of size 32
//     let days:usize=55;

//     //isize-it is for signed value
//     //this means on a 64 bit computer it is of size 64 and on 32 bit computer it is of size 32
//     let years:isize=-15000;


// }


/////////////////////////////////////////////////////////
//Strings and Raw Strings

//whatever strings we used till now are called string literals
//String literals->it is a string whose values or whose text are known by compiler at compile time
//               ->and thats because we write these strings within our source code ,So rust knows them in advance 
//               ->that will sometime be the case ,but not everytime like eg->we may have a progrem where we ask the user to provide some text input ,like their name ,so in that case we will not know the exact string that we'll have to work with until the programs runs


// fn main() {
//     //in below code \n is a special character 
//     println!("Dear Emily,\nHow have you been?");

//     //another special character \t ,this will show a tab where we use it
//     println!("\tOnce upon a time");

//     //below code we will be able to see the quotes in the output
//     println!("Juliet said \"I love you Romeo\"");

//     //So the alternative of below code without using these \ backslahes we can use Raw strings
//     let filepath:&str ="C:\\My Documents\\new\\videos";
//     println!("{filepath}");

//     //Raw strings
//     //In raw strings rust will automatically ignore all special characters
//     let filepath2=r"C:\My Documents\new\videos";
//     println!("{filepath2}");

// }


//////////////////////////////////////////////////////
//Intro to Methods
//Method
//->A method is a functions that lives on a value.It's an action we can ask the value to execute.
//so like when we have a value like number,string ,then we can invoke a method on it,so we can run a function that lives on that value

// fn main() {
//     let value:i32=-15;
//     println!("{}",value.abs());

//     let empty_space:&str="           my content           ";
//     println!("{}",empty_space.trim());

//     //A method can also accept an argument
//     println!("{}",value.pow(2));
//     println!("{}",value.pow(3));
// }


//////////////////////////////////////////////////////
//Floating Point Types

// fn main() {
//     let pi:f64=3.1415987676868777766;
//     println!("The value of pi is {pi}");

//     //methods in floating point types
//     println!("{}",pi.floor());
//     println!("{}",pi.ceil());
//     println!("{}",pi.round());
// }

//////////////////////////////////////////////////
//Formatting floats with format specifier
//Format specifier
//It customizes the printed representation of the interpolated/dynamic value.

// fn main() {
//     let pi:f64=3.1415987676868777766;

//     //here using format specifier we are not changing the visual representation of "pi"  within printed string all we are altering is the digital representaion of "pi" within this printed string below
//     //we impliment above statement by using pi:.2 ->so we use colon and then we use .2 saying that only print the decimal till 2 decimal points
//     println!("The value of pi is {pi:.2}");

//     //alternative
//     println!("The value of pi is {:.2}",pi);
// }

////////////////////////////////////////////////////////
//Type casting in rust
//we impliment this by using "as" keyword

// fn main() {
//     let miles_away=50;

//     //type casting below ,it will work only if you dont make the casting as if the memory of one value is larger or smaller then other,there both range should be able contain that number or any value
//     let miles_away_i8=miles_away as i8;

//     let miles_away=100.329032;
//     let miles_away_f32 = miles_away as f32;

//     //below float to integer is also valid ,but we will loose the value after decimal point
//     let miles_away_int=miles_away as i32;
//     println!("{miles_away_int}");

// }


/////////////////////////////////////////////////////
//Math operations

//operators->it is a symbol that perform some kind of operation 
//operand->the values the symbol acts against 

// fn main() {
//     let addition=5+4;
//     let subtraction=10-6;
//     let multiplication=3*4;
//     let devision=7/2;

//     println!("Addition:{addition},\n subtraction:{subtraction},\n multiplication:{multiplication},\n devision:{devision}");

//     //now for decimal devision
//     let decimal_division=5.0/3.0;
//     println!("{decimal_division}");

//     let remainder=9%2;
//     println!("{remainder}");
// }


/////////////////////////////////////////////////////////
//Augmented Assignment Operator

// fn main() {
//     let mut year=2025;
//     year=year+1;
//     println!("The new year is {year}");

//     //to impliment what we did above we use augmented assignment operator
//     //Augmented assignment operator
//     //->It takes a value ,apply some kind of operation to it ,and then overwrite the orignal variable that they came from ,for this orignanl variable must be mutable
//     let mut year1=2025;
//     year1 += 1;
//     println!("The new year is {year1}");

//     year1 -=5;
//     println!("The new year is {year1}");

//     year1 *= 2;
//     println!("The new year is {year1}");

//     year1 /=4;
//     println!("The new year is {year1}");

// }

///////////////////////////////////////////////////////////
//Boolean
//A boolean occupies 1 byte or 8 bits in memeory

fn main() {
    let is_handsome=true;
    let is_silly=false;

    println!("Handsome:{is_handsome},Silly:{is_silly}");
    
    let age:i32=21;
    let is_younge= age<35;
    println!("{is_younge}");

    println!("{} {}",age.is_positive(),age.is_negative());

    ////////////////////////////////////////////////
    //Boolean Inversion
    println!("{}",!true);

    let age=13;
    let can_see_rated_r_movie=age>=17;
    let cannot_see_rated_r_movie=!can_see_rated_r_movie;
    println!("I am {age} years old.Can I see this scary movie? {can_see_rated_r_movie}");

    ///////////////////////////////////////////////
    //Equality and Inequality operator

    println!("{}","Coke"=="Pepsi");
    println!("{}","Coke"!="Pepsi");
    println!("{}","Coke"=="coke");
    println!("{}","Coke"=="Coke ");
    println!("{}","Coke"=="Coke");

    println!("{}",13==13);
    println!("{}",26.1 == 26.14);

    println!("{}",true==true);

    ////////////////////////////////////////////////////
    //AND logic

    let purchased_ticket=true;
    let plane_on_time=false;
    let making_event=purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected",making_event);

    //OR logic
    let user_has_paid_for_subscription=true;
    let user_is_admin=false;

    let user_can_see_premium_experience=user_has_paid_for_subscription||user_is_admin;
    println!("Can this user see my site? {user_can_see_premium_experience}");


    //////////////////////////////////////////////////////////
    //Character Type
    //A character type represents a single Unicode character
    //Unicode->It is a computing standard for the representation of text for most of the worlds writing systems
    //UTF->stands for Unicode Transformation Formation
    //UTF is like a how much storage can character take ,like UTF-8,UTF-16,UTF-32 and so on

    //so a character must always in single quotes and should only have one character
    let first_initial='B';

    //as we can see even a single character but in double quotes will be considered as string
    let example="h";

    let emoji:char='😂';
    println!("{} {}",first_initial.is_alphabetic(),emoji.is_alphabetic());

    println!("{} {}",first_initial.is_uppercase(),emoji.is_uppercase());


    ///////////////////////////////////////////////////////
    //Array Type

    let numbers:[i32; 6]=[4, 8, 15, 16, 23, 42];

    let apples=["Granny smith","McIntosh","Red Delicious"];
    println!("Length:{}",apples.len());

    let currency_rates:[f64; 0]=[];

    ///////////////////////////////////
    let seasons=["Spring","Summer","Fail","Winter"];

    let first=seasons[0];
    let second=seasons[1];
    println!("The first season is {first} and the second season is {second}");

    let mut data=["he","she","some"];

    //now we can replace the elements in the array
    data[1]="the";
    println!("{}",data[1]);

    /////////////////////////////////////////////////////////
    //Traits
    //1.A trait is a contract that requires that a type support one or more methods.
    //2.Traits establish consistency between types;methods that represent the same behaviour have the same name.
    //3.When a type opts in to honoring a traits requirements,we say the type implements the trait
    //4.Types can very in their implementation but still implement the same trait.

    //1.A type can choose to opting in to implementing a trait.
    //2.A type can implement multiple traits.There are hundreds of traits available in Rust.
    //3.A trait is called an interface or protocol in other programming languages

    ///////////////////////////
    //The Display trait
    //1.The Display trait requires that a type can be represented as user-friendly ,readable string
    //2.The Display trait mandates a format method that returns the string
    //3.when we use the {} interpolation syntax,Rust relies on the format method
    //4.integers , floats,and nooleans all implement the display trait so we are able to interpolate them with curly braces.

    /////////////////////////////////////////////////
    //Debug Traits
    //The goal of debug is to format a given type into a programmer facing string for the purposes of debugging

    let seasons1=["Spring","Summer","Fail","Winter"];

    //below code will call the method behind the scene,But now its going to be the 'format' method mandated by the debug trait,while the regular braces is the 'format' method mandated by the display trait
    //array do impliment the debug trait
    //below in curly braces {:?} is how we are able to output all array ,it will show error if we only use {} curly braces ,that is what we have here is debug trait
    //:? ,:?# is what we call the format specifier
    println!("{:?}",seasons1);

    //it also works same as above 
    println!("{seasons1:#?}");

    /////////////////////////////////////////////////////
    //dbg macro
    //like all macro in rust debug macro also ends with an exclamation point(!).
    //dbg macro exist to print and return the value of a given expression for quick and dirty debugging
    //Behind the scene what the dbg macro does is it uses the Debug traits 'format' method to output several helpful details

    //this will also show the output like println! macro but it will also show folder name,file name ,line number and output
    dbg!(2+2);
    dbg!(seasons1);

    /////////////////////////////////////////////////////////////
    //Tuple Type
    //Tuple is a collection type that can contain multiple elements,each of which is assigned an index position reflecting its order in line
    //The difference b/w the two data types is that a tuple supports different types for its values

    let employee=("Molley",32,"Marketing");

    // let name =employee.0;
    // let age=employee.1;
    // let departement=employee.2;
    //implimenting just above commented part in single line
    let (name,age,departement)=employee;

    println!("Name:{name},age:{age},departement:{departement}");

    println!("{employee:?}");

    /////////////////////////////////////////////////////////
    //Ranges and Range Iteration

    //below the range is from 1 up to but not including 31
    let month_days=1..31;
    println!("{month_days:?}");

    //now to include the last value
    let month_days1=1..=31;
    println!("{month_days1:?}");

    //Iteration
    for number in month_days1  {
        println!("{number}");
    }

    let letters='b'..'f';
    for letter in letters  {
        println!("{letter}");
    }

    let colors=["Red","Green","Yellow"];
    for col in colors  {
        println!("{col}");
    }

    ////////////////////////////////////////////////////////////
    //Generics
    //A generic is a type argument 
    //type is a kind of data like value =5 so it is a data ,but like type=i32 so it is kind of data,its a catogory of data
    //generic exist so we can provide type like (Type-i32) this one their own argument

    //so the way to support the possibility of multiple types is what we call generic
    //A Generic is a name for an abstract type It acts as a placeholder for a future actual type

    //these ranges type below uses something called generic
    let month_days2=1..31;
    let letters2='b'..'f';

    //below generic syntax->std which is short for standard which refers to standard library,ops this is short for operators,we can think of double colon :: like going deep in rust directory and then we write range like Range<i32> so a i32 inside is what we call the generic output
    let month_days2:std::ops::Range<i32>=1..31;

}

