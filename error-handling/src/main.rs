use std::fs::File;
use std::process;
use std::io::{stdin, Read};

fn main() {

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The process Module and the exit Function

    //below the 0 means there is no error,so here we are treminating/exiting the program and indicating that there are no error message
    // process::exit(0);

    //below the 1 means there is a error,and this code will terminate the program,nothing after this will execute
    // process::exit(1);



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Standard Error (eprintln! Macro)
    //this macro also output the text,this macro print messages to whats called the standard error
    println!("Some status update");
    eprintln!("Some error message");




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Opening a File and Reading the File

    println!("Please enter the name of the file you'd like to read");
    let mut input =String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input.The error was {error}");
        process::exit(1);
    }

    //this below code a file and then models it as an existing File struct or as an file instance
    let mut file =match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file.The error was {error}");
            process::exit(1);
        }
    };
    let mut file_contents =String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    //below Err is same as Result::Err ,which is a type of read_operation variable
    if let Err(error) = read_operation {
        eprintln!("Something went wrong opening a file as a string.The error was {error}");
        process::exit(1);
    }

    println!("{}",file_contents);
}
