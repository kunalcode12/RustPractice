use std::io;

fn main() {
    let mut full_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    //push() method will only add one character at the end
    full_name.push(' ');

    full_name.push_str(&last_name);
    println!("{full_name}");

    let first_name = String::from("Sylvester");

    let full_name = first_name + &last_name;
    println!("{full_name}");



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The format! Macro
    //It is similar to println! ,but instead of printing a string ,it returns a formatted string ,including any interpolated contents that we inject within it
    let first_name1 = String::from("Sylvester");
    let last_name1 = String::from("Stallone");

    //below ownership is does not transfer
    let icon =format!("{first_name1} {last_name1}");
    println!("{icon}");



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Common String Methods (trim, casing, replace, split)

    let mut music_geners = "    Rock, Metal, Country, Rap    ";
    println!("{}",music_geners.trim());

    //below method will only remove the spacing from the beginning
    println!("{}",music_geners.trim_start());
    //opposite of just above
    println!("{}",music_geners.trim_end());

    music_geners=music_geners.trim();
    println!("{}",music_geners);

    //this below method will create a new heap alloacated string
    println!("{}",music_geners.to_uppercase());
    println!("{}",music_geners.to_lowercase());

    //below method is going to replace all matches of a given character or a pattern ,with another another character
    //below is also the new heap allocated string
    println!("{}",music_geners.replace("a", "@"));

    //here collect will add all the strings in the vector which we defined as a type below Vec<&str>
    let genres:Vec<&str> = music_geners.split(", ").collect();
    println!("{:?}",genres);




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Collecting User Input with read_line Method

    let mut name: String = String::new();
    println!("What is your name?");

    //first method to prevent yellow lines as readline return an enum ,and we are only using one part of enum to print the data ,as another one is for error
    // io::stdin().read_line(&mut name).expect("Failed to collect input from the user");

    //second method
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}",name.trim()),
        Err(message) => println!("There was an error: {message}"),
    } 

    
}
