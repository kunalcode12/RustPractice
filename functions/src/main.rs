
fn main() {
    open_store();
    bake_pizza();
    swin_in_profit();

    //we can also call same function many times in main
    swin_in_profit();
    swin_in_profit();
    swin_in_profit();

    open_store1("Brooklyn");
    open_store1("Queens");

    bake_pizza1(5,"pepperoni" );

    let result= square(5);
    println!("The square of 5 is {result}");

    let result1=square1(10);
    print!("The square of 10 is {result1}");

    let result2=unit();

    ///////////////////////////////////////////////////////
    //Blocks in Function
    let multiplier=3;

    //Block
    let calculation={
        let value=5+4;
        value*multiplier
    };
    println!("{calculation}");


    //Practice
    apply_to_jobs(10, "developer");
    let condition= is_even(8);
    println!("Is the number even:{condition}");

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zebra"));

}

/////////////////////////////////
//Functions

//the below function will run if we call this in above main function
fn open_store() {
    println!("Opening my pizza store");
}

fn bake_pizza() {
    println!("Baking a pizza");
}

fn swin_in_profit() {
    println!("So much $$$, so little time");
}

/////////////////////////////////////////////
//Parameters and arguments

fn open_store1(neighborhood:&str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza1(number:i32 ,toppings:&str) {
    println!("Baking {number} {toppings} pizzas");
}

//////////////////////////////////////////////////
//Explicit Return Values

//so what this ->i32 means that we are saying below function will accept a paramater called number that will be of type i32 and it will return a value back that will also be of type i32
//so we can return anything from a function it is not dependent on the parameter but whatever we are returning should be same as the return type we are mentioning here in function
fn square(number:i32)->i32 {
   return number*number;
}

////////////////////////////////////////////////////
//Implicit Return Values
//now in rust function we can automatically return the values ,so whatever the last line's value will automatically be the return value

// so now we just remove the semicolon at the last ,so rust will know that this is the value which we need to return
fn square1(number:i32)->i32 {
    number*number
}

//////////////////////////////////////////////////////
//The Unit as a Return Type
//A Unit is an empty tuple,a tuple without values
fn unit() {}

///////////////////////////////////////////////////////////////////////
//Practice
/*
Define an apply_to_jobs function that accepts a
'number' parameter (an i32) and a 'title' parameter
(a string). It should print out the string:
"I'm applying to {number} {title} jobs".
 
Example:
apply_to_jobs(35, "Rust Developer")
-> "I'm applying to 35 Rust Developer jobs"
 
Define an is_even function that accepts a 'number'
parameter (an i32). The function should return a true
if the number is even and a false if the number is
odd.
Examples:
is_even(8) -> true
is_even(9) -> false
 
Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains
 
Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/

fn apply_to_jobs(number:i32,title:&str) {
    println!("I'm applying to {number} {title} jobs");
}

fn is_even(number:i32)->bool {
    number%2==0
}

fn alphabets(text:&str)->(bool,bool) {
    (text.contains("a"), text.contains("z"))
}