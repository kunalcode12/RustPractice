//bellow fn  tells rust that we are beginning a function declaration
//every rust program must have a function called main like bellow ,there can be function with other name ,but there should atleast one main named function
//the main function should always in lowercase and the main function is the entry point to the program, as rust will automatically execute the main function

fn main() {
    //bellow println!() is the macro
    println!("Hello World I am a Rust developer");
}

//////////////////////////////////////////////
//To run from terminal
//1.rustc filename, eg->rustc main.rs
//2.there would be creation of exe file after above step so run-> .\filename.exe ,eg->.\main.exe

////////////////////////////////////////////
//To Format the code of rust
//below code only target one rust file
//1.write in terminal-> rustfmt filename ,eg->rustfmt main.rs

//Now to format all the code in the whole file we can use cargo tool ->this tool will automatically run this above command in every rust file in our project
//1.write in terminal ->cargo fmt ,but you should be in most outermost directory like you should be at topmost folder

/////////////////////////////////////////////////////////
//command ->cargo build  ,this command will use rust compiler to build the complete project

//we can build or compile the final application in one of two mode 
//1.Debug->by default (command->cargo build) runs in debug mode 
//        ->it is fast and unoptimized build
//        ->mainly used in development phase

//2.Release->It is optimized (command->cargo build --release) for runtime performance,so it does not inclue any of the debug information which result in smaller final executable
//         ->It is mainly for final released program


/////////////////////////////////////////////
//command->cargo clean , it will delete the target folder ,like everything which was created after cargo build and we can restart everything 

//////////////////////////////////////////////
//command->cargo run , this will build the executable (target folder) and also run the complied code from the executable 

////////////////////////////////////////////
//command->cargo check , this will checks the source code for any compiler violations,but it does not produce any actual executable