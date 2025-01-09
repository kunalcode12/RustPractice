// fn main() {
//     //to write variable name there should be no space in characters ,like variable name can be like just ->value ,and if long name then it should be written like ->value_of_x
//     let apples = 50;
//     let oranges=14 + 6;
    

//     // let fruits =apples + oranges; // this code is always showing this warning ,so we can also remove this warning by using underscore_ before variable name
//     //through below code symbol complier can know that it should not give warning for this
//     let _fruits=apples+oranges;

//     //so in the below code the apples value will be shown in the place of {} ,as it is a syntax to impliment it,and it is also in order ,like first {} will have first value after , in println! macro and so on
//     //apples-10 ,this will also work
//     println!("This year ,my garden has {} apples and {} oranges",apples-10,oranges);

//     //like in below code apples is at place/index 0 and oranges at 1 so we can also do like {0} {1} so through this we can use same values many time without writing that again and again after comma in below code 
//     println!("This year ,my garden has {0} apples and {1} oranges and the apples count {0} ",apples,oranges);


//     //another way to impliment which we implemented just above 
//     println!("This year ,my garden has {apples} apples and {oranges} oranges");


//     /////////////////////////////////////////////////////////
//     // below gym_reps is immutable by default
//     // let gym_reps=10;

//     //now to declare the mutable varible 
//     let mut  gym_reps=10;
//     println!("I plan to do {gym_reps} reps");

//     gym_reps = 15;
//     println!("I now plan to do {gym_reps} reps");

// }


///////////////////////////////////////
//Variable shadowing
//it means redeclaring a variable.the orignal variable is "replaced" by the new one.
//the second declaration becomes the active variable
//most common use case is when we want to perform one or more transformations on a value ,but reuse the same variable name throughout the process ,as that name represent the same value ,but in different formats or stages 
// fn main(){
//     let grams_of_protein="100.345";
//     //so from here till the next same declaration of above variable ,its value is above one 

//     //below we transformed above variable->as when we use the same variable with let again ,then is what we call varaible shadowing,it invalidates the previous declaration of the variable
//     let grams_of_protein=100.345;
//     //so from here till the next same declaration of above variable ,its value is above one 


//     //another transformarion
//     let grams_of_protein=100;
//     //so from here till the next same declaration of above variable ,its value is above one 


// }

/////////////////////////////////////////////
//Scope
//It is the boundary or region of code where a name is valid

// fn main() {
//     let coffee_price=5.99;

//     {
//         let coffee_price=1.99;
//         println!("The Price is {coffee_price}");

//         //below variable is not acesible outside this block
//         let cookie_price=1.99;

//     }
    
// }

///////////////////////////////////////////////
//Constants
//It is permanently immutable.
//It is different from an immutable variable ,it has two differences
//1.The variables are limited to a function scope ,while the constants can be declared at any scope ,including at the file level,it allows them to reused across multiple functions
//2.Constant value must be known at complile time,variable value is defined at runtime,a contants value must be known when the rust compiler actally builds your program

//when declaring constant we must manually provide the type and the actual value of the constant
// const TAX_RATE:f64=7.25;

// fn main() {
//     let income=100000;
//     println!("My Income is {income} and my tax rate is {TAX_RATE}");
// }


/////////////////////////////////////////////////////
//Type Alias
//It is a nickname/alternate name that we can assign to an existing type
//the benifit is that we can provide some additional context on what that type represents

//below is but we call type alias
// type Meters=i32;

// fn main() {
//     let mile_race_length: Meters=1600;
//     let two_mile_race_length:Meters=3200;
//     println!("A one mile race is a {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long");

// }


/////////////////////////////////////////////////////////
//Compiler directive
//the word directive means an instruction or an command 
//It is an annotation that tells the compiler how to parse the source code
//a complier directive is an instruction to a compiler ,it is a metadata that we add that customizes how the compiler thinks and operates

#![allow(unused_variables)] //this is kind of like global diractive

type Meters=i32;

//below directive allows a behaviour that the compiler would otherwise would show a warning about
// #[allow(unused_variables)] //it will allow the unused variables for the just next line function 
fn main() {
    //below directive allows a behaviour that the compiler would otherwise would show a warning about
    //below directive applies to its next line only
    // #[allow(unused_variables)]
    let mile_race_length: Meters=1600;

    // #[allow(unused_variables)]
    let two_mile_race_length:Meters=3200;

}