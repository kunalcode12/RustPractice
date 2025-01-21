use std::fmt::Error;
use std::fs;
use std::process;
use std::io::{self ,stdin, Read};

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

    // println!("Please enter the name of the file you'd like to read");
    // let mut input =String::new();

    // let user_requested_file = stdin().read_line(&mut input);

    // if let Err(error) = user_requested_file {
    //     eprintln!("Something went wrong collecting user input.The error was {error}");
    //     process::exit(1);
    // }

    // //this below code a file and then models it as an existing File struct or as an file instance
    // let mut file =match File::open(input.trim()) {
    //     Ok(file) => file,
    //     Err(error) => {
    //         eprintln!("Something went wrong reading the file.The error was {error}");
    //         process::exit(1);
    //     }
    // };
    // let mut file_contents =String::new();
    // let read_operation = file.read_to_string(&mut file_contents);

    // //below Err is same as Result::Err ,which is a type of read_operation variable
    // if let Err(error) = read_operation {
    //     eprintln!("Something went wrong opening a file as a string.The error was {error}");
    //     process::exit(1);
    // }

    // println!("{}",file_contents);

    //copied all these above commented code to the function after main




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Propagating Errors
    let file_result = read_file();

    match file_result {
        Ok(contents) =>println!("{contents}"),
        Err(error) =>{
            eprintln!("There was an error: {error:?}");
        }
    }



    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The ? Operator (The Try Operator)




    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Using ? with Option

    let mut animals =vec!["Giraffe","Monkey","Zebra"];
    println!("{:?}",length_of_last_element(&mut animals));

}

// fn read_file()->Result<String,io::Error> {
//     println!("Please enter the name of the file you'd like to read");
//     let mut input =String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         //here we are gonna propogate the error above
//         // eprintln!("Something went wrong collecting user input.The error was {error}");
//         // process::exit(1);

//         //return Result::Err(error) same sa below
//         return Err(error);
//     }

//     //this below code a file and then models it as an existing File struct or as an file instance
//     let mut file =match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             // eprintln!("Something went wrong reading the file.The error was {error}");
//             // process::exit(1);

//             return Err(error);
//         }
//     };
//     let mut file_contents =String::new();
//     let read_operation = file.read_to_string(&mut file_contents);

//     //below Err is same as Result::Err ,which is a type of read_operation variable
//     if let Err(error) = read_operation {
//         // eprintln!("Something went wrong opening a file as a string.The error was {error}");
//         // process::exit(1);

//         return Err(error);
//     }

//     // println!("{}",file_contents);
//     Ok(file_contents)
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//The ? Operator (The Try Operator)

//Refactoring just above code
// fn read_file()->Result<String,io::Error> {
//     println!("Please enter the name of the file you'd like to read");
//     let mut input =String::new();

//     //so now if returns an error variant the ? will terminate early and return it ,so we can remove the code below
//     stdin().read_line(&mut input)?;

    
//     //this below code a file and then models it as an existing File struct or as an file instance
//     // let mut file = File::open(input.trim())?;

//     // let mut file_contents =String::new();
//     // file.read_to_string(&mut file_contents)?;

//     //alternative of just above
//     let mut file_contents =String::new();
//     File::open(input.trim())?.read_to_string(&mut file_contents);

    
//     Ok(file_contents)
// }

//refactoring just above code
fn read_file()->Result<String,io::Error> {
    println!("Please enter the name of the file you'd like to read");
    let mut input =String::new();

    //so now if returns an error variant the ? will terminate early and return it ,so we can remove the code below
    stdin().read_line(&mut input)?;

    //below code will return the Result containing a String representing the file's contents
    fs::read_to_string(input.trim())
    
}


fn length_of_last_element(input:&mut Vec<&str>) ->Option<usize> {
    //below as it will be Option type ,so below is if there is a None variant/vector is empty return early and if you have the Some variant indicating that we were successfully able to pop off some element ,then last element will be the associated data from the Some variant
    let last_element = input.pop()?;
    // Option::Some(last_element.len());
    Some(last_element.len())
}