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

fn main() {
    let pi=3.14159;
}