///////////////////////////////////////////////
//Ownership
//it is a set of rules that the compiler checks for to ensure the program will be free of memoer errors.
//The rust compiler does not compile the program if an ownership rule is violated.
//Ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that is no longer needed.


//What is ownership?
//1.The Owner is who/what is responsible for cleaning up a piece od data when it is no longer in use.
//2.Every value in a Rust program has one owner
//3.The owner can change over the course of the program but there is only 1 owner for a value at a time.
//4.The owner is usually a name,a variable can be an owner.,a parameter can be an owner
//5.Ownership also extends to composite types that own their elements


// fn main() {
//     let age=33;
//     let something=true;

//     {
//         let is_handsome=true;

//     }//is_handsome goes out of scope here

//     println!("{age}");
//     println!("{something}");

//     //age variable exists here
// } //age variable goes out of scope here


///////////////////////////////////////////////////////////////
//Copy Trait
//It mandates that a type can be copied which means that a full duplicate can be created from it

fn main() {
    let time=2025;

    //copy trait
    let year=time;

    println!("The time is {time}.It is the year {year}");

    
    /////////////////////////////////////////////////////////
    //String Type

    //below is the built in string type in rust
    //below type is not stored on either stack or heap rather it is embedded directly into the binary executable that the Rust compiler produces 
    //and the reason for just above statement is ->the value is already known at compile time
    //below method is good when we know exact string value at compile time
    let food="pasta";

    //Now the another type of string
    //below string type is mainly used dynamic strings,means this strings size we cant predict at compile time ,or strings that are going to change over the case of the program
    //below string type is mutable
    //so below new is not the method ,it is a plain function but one that lives within the string namespace

    //below the text variable is the owner of the string and thus the 'text' variable is responsible for cleaning it up which means at the end of the main function when the 'text' variable goes out of scope so as the owner of this data ,it is responsible for deallocating the 'text' contents from the heap
    let text:String= String::new();

    //below the 'from' function is also found within the String namespace
    //'From' allows us to pass in an argument is a string literal ,a 'str' ,it is going to create a string on a heap but based on the data which i am providing
    let candy: String=String::from("KitKat");


    ////////////////////////////////////////////////////////////
    //The push_str Method on a String Type
    //method->push_str
    //what it does is it adds content to the end of a heap-allocated string.
    //like ->"hello" ,so this string/text that rust is going to hardcode directly into the binary executable and the program is actually working with a reference to that text from the executable.
    //so we cant mutate something that is written in place in the executable file,but we can mutate a string on a heap

    let mut name=String::from("Boris");
    println!("{name}");

    //so now it is still in the capacity of memeory assigned to name variable in heap and stack so that is why below code is working
    name.push_str(" pask");
    println!("{name}");

    //now if the capacity increases then to support the expanded size,so now the text will we moved to another heap loacation with a capacity greater then before and then that stack entry for that variable will be automatically updated ,so it will store s reference to the new heap location with the increased capacity then before
    name.push_str("haver");

    

    /////////////////////////////////////////////////////////////////
    //Moves and ownership
    //Moves->A move is the transfer of ownership from one owner to another.

    let person=String::from("Boris");

    //this below code will operate completely differently from the integer example.A heap allocated String does not impliment the copy trait ,and therefore Rust will not make a copy of this heap data when 'person' is reassigned to 'genius' as that would create a duplicate of the heap's text data,which we can generally assume is going to occupy more memory that something on the stack
    //so when we assign 'genius' to 'person' Rust copies the previous stack entry,so it copies the reference to heap data,length and the capacity from the 'person' stack entry to a new 'genius' stack entry,But Rust does not actually copy the string's text content on the heap,so there is still one piece of heap data,but now we have two references to it on the stack
    //so now fro ownership ,given that there is one piece of data on the heap,who is responsible for cleaning it up for deallocating it ,is it the 'person' variable or is it the 'genius' variable ,so to clear that it is going to be 'genius variable.
    //according to just above statement ,why 'genius' varaible->as when we assign 'person' to 'genius' Rust moves ownership from 'person' to 'genius' ,so 'genius' becomes the owner which means that 'person' becomes invalid means it goes out of scope ,means we cannot use the varaible 'person' varaible after line 97.
    //so when ownership moves from one variable to another ,like from 'person' to 'genius' ,its as if the orignal variable never existed.so at the end of the function scope ,rust only has to drop the 'genius' variable out of scope and it would clean up the corrosponding haep text.
    let genius=person;

    //so below code will show error as it does not exist at this point for more check on above statements
    // println!("My name is {person}");



    ////////////////////////////////////////////////////////////////
    //The Drop function
    //more context on what happens when a variable goes out of scope and as an owner cleans up some heap allocated memory
    //So Rust has a function built in called Drop
    //Drop function->It deallocates the memory on the heap,Rust automatically calls the 'drop' function at the end of a scope,at the closing curly braces of function like main function ,and it automatically passes in every variable one by one that is the owner of some heap data 
    //for drop function the memory should be heap memory it does not work on stack memory
    //as rust does this automatically ,we can also do manually,we can explicitly invoke a drop function in our function to invalidate a variable and deallocate its corresponding memory before the function is done running.

    let person1=String::from("Boris");

    //so after using drop below the person1 variable is invalid and cannot be used
    //so after drop below if we tried to use 'person1' again,it would be referring to a heap location that no longer has a valid data.
    drop(person1);

    //Invalid
    //println!("{person1}");
    //let genius1=person1;



    //////////////////////////////////////////////////////////////////
    //The Clone Method
    //so to actually make the copy of heap data ,we have to tell it explicitly,manually when we do want to duplicate heap data
    //Clone->So to force a copy of existing heap data and thus not move ownership and avoid the issue we're seeing here,we can invoke a method called 'clone' on the orignal value.
    let person2=String::from("Boris");
    let genius2=person2.clone();

    //we are also able to access person2 varible here as we only cloned the person2 above so its ownership is still to it ,as there is no 'move' rather there are now two owners for two seperate,distict but equal pieces of data on the heap.
    println!("This is {person2}");




    //////////////////////////////////////////////////////////////////
    //Refrences and Borrowing


}