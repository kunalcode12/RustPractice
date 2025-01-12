///////////////////////////////////////////////////////////////////
//Slices
//A collection type is one that can hold multiple values .Arrays,tuples, and strings are collection types
//A slice is a reference to a portion/sequence of a collection type.Its a subcategory of referece
//A string slice is a reference to a sequence of characters from string .
//An array slice is a reference to a sequence of elements from an arry.
//As a reference ,a slice does not take ownership of the collection.

fn main() {
    let action_hero=String::from("Arnold Schwarzenegger");

    //so in below method we can only borrow the whole String value,we cannot borrow a portion or chunk or slice of the orignal text
    let string_reference=&action_hero;
    println!("{string_reference}");

    //Now for borrowing slices or chunks or some part of the orignal variable
    //below in [] this we are spacifying a byte interval not a character range or interval
    //so [0..6] will print till 5 ,not till 6
    let first_name: &str=&action_hero[0..6];
    println!("{first_name}");

    let last_name=&action_hero[7..21];
    println!("{last_name}");



    /////////////////////////////////////////////////////////////
    //String Slices and String Literals

    //below action_hero1 is not the owner of any kind of data as it is &str type so it is a reference which means 'action_hero' is not owning any of the text,its just providing a memory address to a portion of text somewhere else and that happens to be the whole orignal text in the executable so it is equivalent of borrowing a complete house
    let action_hero1="Arnold Schwarzenegger";

    //we use action_hero1 reference as the basis for creating a seperate independent reference
    //below first_name 1 variable is not dependent on the 'action_hero1' variable existing after this line,The fisrt_name1 variable relies on 'action_hero1' once to identify the chunk in th ememory and then use it as the basis to create its own independent reference to that same chunk of memeory ,but just to the first 6bytes instead of a whole thing like in above 'action_hero' variable
    let first_name1: &str=&action_hero1[0..6];
    println!("{first_name1}");

    let last_name1=&action_hero1[7..21];
    println!("{last_name1}");


    /////////////////////////////////////
    let first_name2={
        let action_hero2="Arnold Schwarzenegger";
        &action_hero2[0..6]
    };

    println!("{first_name2}");



    //////////////////////////////////////////////////////////////////
    //String Slice Lengths
    //The length of the string slice refers to a count of its bytes,not its characters

    let food="pizza";
    println!("{}",food.len());

    let pizza_slice=&food[0..3];
    println!("{}",pizza_slice.len());

    ///////////////////////////
    let food1="😊";

    //so below code in this output=4 ,so this means ,the the value of food1 is visually as one character,but behind the scenes it ocupies 4 bytes in memory and that is the 4 bytes in memory that loads in the binary executable file's content
    //
    println!("{}",food1.len());

    //so below it will show error as we cannot provide a reference to a invalid byte sequence that does'nt provide a full valid character
    // let pizza_slice1=&food1[0..3];
    // println!("{pizza_slice1}");


    ////////////////////////////////////////////////////////////////
    //Syntactic Shortcuts
    let action_hero3="Arnold Schwarzenegger";

    // let first_name3: &str=&action_hero3[0..6];
    //below is the shortcut of above code 
    let first_name3: &str=&action_hero3[..6];
    println!("{first_name3}");

    // let last_name3=&action_hero3[7..21];
    //below code is shortcut of above code
    let last_name3=&action_hero3[7..];
    println!("{last_name3}");

    //below syntax means start from beginning and go to the end
    let full_name=&action_hero3[..];
    println!("{full_name}");


    //////////////////////////////////////////////////////////////////
    //String Slices as Function Parameters

    let action_hero_name=String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero_name);

    let another_action_hero_name="Sylvester Stallone";

    //this below code will give error as the function is expecting a reference to a full string &String ,but we are passing in a string slice
    // do_hero_stuff(another_action_hero_name);

    //so now below function have a parameter of type &str which accepts both reference string like 'another_action_hero_name' and also the whole string like 'action_hero_name' 
    do_hero_stuff_2(&action_hero_name);
    do_hero_stuff_2(&another_action_hero_name);



    /////////////////////////////////////////////////////////////
    //Array Slices

    let values=[4, 8, 15, 16, 23, 42];
    let my_slice=&values[0..3];
    println!("{my_slice:?}");

    let my_slice=&values[2..4];
    println!("{my_slice:?}");


    /////////////////////////////////////////////////////////////////
    //Deref Coercion with Array Slices

    let values1=[4, 8, 15, 16, 23, 42];

    //so below code is not tecnically slice but a whole reference to values1 variable
    let regular_reference=&values1;

    let slice_of_three=&values1[..3];

    print_length(regular_reference);

    //so this below function will show error if we dont provide the whole array reference as we already described the parameter type of that function to accept that whole array reference
    // print_length(slice_of_three);

    //more generic function which will work for both types
    print_length1(regular_reference);
    print_length1(slice_of_three);


    //////////////////////////////////////////////////////////////
    //Mutable Array Slices
    //Rust allows mutable slices of arrays

    let mut my_array=[10, 15, 20, 25, 30];

    //below variable is mutable reference to an array and more specifically mutable slice,which you can think of as a subcategory of references
    //it is a mutable reference to a portion of the greater whole
    let my_slice_array=&mut my_array[2..4];

    println!("My Slice: {:?}",my_slice_array);

    my_slice_array[0]=100;
    println!("My Slice: {:?}",my_slice_array);

    //so our orignal my_array varaible will also get afftected
    println!("My Array: {:?}",my_array);

}

fn do_hero_stuff(hero_name:&String) {
    println!("{hero_name} saves the day");
}

fn do_hero_stuff_2(hero_name:&str) {
    println!("{hero_name} saves the day");
}

fn print_length(reference:&[i32; 6]) {
    println!("{}",reference.len());
}

fn print_length1(reference:&[i32]) {
    println!("{}",reference.len());
}