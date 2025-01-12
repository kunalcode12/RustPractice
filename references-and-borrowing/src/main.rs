
fn main() {
    let mut current_meal=String::new();
    
    //so below we dont have to declare a new variable to store its new valus ,and it is shorter method to change the string without moving the ownership
    add_flour(&mut current_meal);

    //below in function if directly provide current_meal ,it will be a type mismatch as current_meal is a string but below function accept a reference of a string
    show_my_meal(&current_meal);


    ///////////////////////////////////////////////////////////////
    //Multiple Immutable References

    //A value in program can have any number of immutable references at the same time
    let car=String::from("Red");
    let ref1=&car;
    let ref2=&car;

    println!("{ref1} and {ref2} and {}",car);

    //Mutable Reference Restrictions

    //A value can only have a single mutable reference at a time,
    //so if we declare a single mutable reference ,we cannot have any other references to that same value coexist at the same time
    //so once we declare a single mutable reference,we cannot declare a second reference at the same time, regardless of whether that second reference is mutable or immutable,it does not matter
    let mut car1=String::from("Blue");

    //below is a mutable reference
    let ref3=&mut car1;
    ref3.push_str(" and Silver");

    //so once we have one single mutable reference ,there can be no more reference
    let ref4=&car1;

    //so now if i dont use ref3 at all then we can use ref4 as there is no issue if the value is not changing or changing before the declaration of ref4
    // println!("{ref3}");

    println!("{ref4}");



    ////////////////////////////////////////////////////////////////
    //Ownership with Immutable and Mutable References

    //we know that immutable references impliment the copy trait
    let mut coffee=String::from("Mocha");
    let a=&coffee;

    //so what happen below is ,so the 'a' variable remains valid and it remains the owner of its own immutable reference and 'b' represents a copy ,a seperate entry on stack ,a seperate immuatable reference that it is the owner of ,so we can use both b and a
    let b=a; //this is same as let b:&String=&coffee;

    println!("{a} and {b}");

    //so now above copy trait is not valid for mutable reference
    let c=&mut coffee;

    //so now below ownership of 'c' moves from 'c' to 'd' so now c ownership is invalid and cant be used as mutable reference does not have copy trait
    let d=c;
    // println!("{c} and {d}");




    ////////////////////////////////////////////////////////////////////
    //Dangling References
    //It is a pointer to a memory address that has been deallocated.
    // let reference=create_city(); //see the explanation at the function



    ////////////////////////////////////////////////////////////
    //Ownership with Arrays and Tuples

    let registrations=[true, false, true];

    //A boolean impliments the copy trait,so whenever we extract the first element ,what happens is Rust creates a full copy of the boolean and assigns it to the 'fisrt' variable
    let first=registrations[0];
    println!("{first} and {registrations:?}");

    //Now we see what will happen if have an array of heap owned data like Strings
    let languages=[String::from("Rust"),String::from("Javascript")];

    //below it will not work as a memeory in heap does not implimenta copy trait and the ownership will move from array to 'first1' variable so that is not possible here
    // let first1=languages[0];

    //so to actually access that languages array value we can use like clone(),& and all
    let first1=&languages[0];
    println!("{first1} and {languages:?}");


    ///////////////////////////////////////////////////////////////////////////////////
    //Practice
    let mut trip=start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");

    visit_new_york(&mut trip);
    trip.push_str(" and ");

    visit_boston(&mut trip);
    show_itinerary(&trip);

    // println!("{trip}");

}

//different kind to pass in parameter
//meal:String
//mut meal:String
//meal:&String
//meal:&mut String

fn add_flour(meal:&mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal:&String) {
    println!("Meal steps: {meal}");
}

// fn create_city()->&String {
//     let city=String::from("New York");

//     //below is what we call the dangling reference,as above city variable becomes the owner of that String so 'city' is the name that is responsible for deallocating that heap memory , so this happens when city variable goes out of scope which is the end of this function,so below we are returning a reference to that spot in the heap memory that will no longer be holding the String after the function ends
//     &city
// }


////////////////////////////////////////////////////////////////////
/*
Let's model a road trip!
 
Define a `start_trip` function that creates and returns
a String of "The plan is..."
 
Invoke the `start_trip` function in `main` and save its
return value to a `trip` variable.
 
We want to pass the String to three separate functions
that will mutate the String without transferring ownership.
 
Define a `visit_philadelphia` function that concatenates
the text "Philadephia" to the end of the String. Invoke
the function in `main`. Then, invoke `push_str` on the String
to concatenate the content " and " to the end. Mak sure to
include the spaces.
 
Define a `visit_new_york` function that concatenates the
text "New York" to the end of the String. Invoke the function
in `main`. Repeat the previous logic to concatenate " and "
to the end of the String.
 
Define a `visit_boston` function that concatenates the
text "Boston." to the end of the String. Invoke the function
in `main`. Concatenate a period to the end of the
String/sentence.
 
Define a `show_itinerary` function that will print out
the final version of the String. Find a way to do so
without transferring ownership.
 
Invoke `show_itinerary`. The final output should be:
 
"The plan is...Philadelphia and New York and Boston."
*/

fn start_trip()->String {
    String::from("The plan is...")
}

fn visit_philadelphia(value:&mut String) {
    value.push_str("Philadephia");
}

fn visit_new_york(value:&mut String) {
    value.push_str("New York");
}

fn visit_boston(value:&mut String) {
    value.push_str("Boston.");
}

fn show_itinerary(value:&String) {
    println!("{value}");
}