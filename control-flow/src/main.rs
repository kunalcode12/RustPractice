
///////////////////////////////////////////////////////////
//Control flow
//Control flow refers to how a program will execute (the "flow" of code).


fn main() {
    let some_condition=true;

   if some_condition {
       println!("This line will be output");
   }

   if some_condition {
       println!("This line will NOT be output");
   }

   /////////////////////////////////////////////////////
   //Else if Statement
   let season="summer";

   if season == "summer"{
    println!("School's out!");
   } else if season=="winter" {
       println!("Brr,so cold!");
   }else if season=="fall" {
       println!("Leaves falling!");
   }else if season=="spring" {
       println!("Lots of rain!");
   } else {
       println!("Invalid data");
   }

   //////////////////////////////////////////////////
   //Assigning result of if statement to variable
   even_or_odd(17);



   ///////////////////////////////////////////////////////
   //Match statement
   //It is like a switch case in other programming languages
   //The match statement can react to all possible variants of a value

   let evoloution=true;

   //in bellow code evoloution ,rust will check every possible value for eveolution and if not all value is covered inside the match then it will show error,so all values should be covered
   match evoloution {
       //bellow is pattern/arm->it is one possible option to compare the match value against
       //so bellow is like a value which could be the value of evolution above ,and if it is then below block of code in {} will be executed
       true=>{
        println!("The value is true");
       }

       false=>{
        println!("The value is false");
       }
   }

   //we can also assign the match statement to variable
   let value= match evoloution {
       true=>20,
       false=>40,
   };

   println!("The value is {value}");

   //////////////////////////////////////////////////////
   //Underscore in a Match arm
   let seasons="winter";

   match seasons {
    //all the arms should return the same data types 
       "summer"=>println!("Schoo's out!"),
       "winter"=>println!("Brr,so cold"),
       //so now compiler need all the possible values for seasons but as seasons variable is a tring so it has infinite values ,so to make this code work we will use underscore _ below,so here underscore is like a else statement 
       _ =>println!("Lots of rain!")
   }


   //////////////////////////////////////////////////////////////
   //The match Statement with Multiple Values and Conditionals

   let number=8;

   match number {
    //below to check multiple values at a same time (|) is called pipe ,if the value is any of these then that block/arm work will get implimented
       2 | 4 | 6 | 8 =>println!("{number} is even"),
       1 |3 | 5 =>println!("{number} is odd"),
       _ =>println!("Unknown"),
   }

   //better method then above
   match number {
       value if value%2 == 0 =>println!("{value} is an even number"),
       x if x%2 != 0 =>println!("{x} is an odd number"),
      // _ =>println!("Unknown")
      //Implimentation of just above method by different approach
      _ =>unreachable!(),
   }


   ////////////////////////////////////////////////////////////
   //The loop and break Keywords
   //loop keyword declares a block of code that rust will continue executing oven and over,By default there is no endpoint 
   //so to terminate the loop and to prevent infinite loop we will use "break" keyword
   
   let mut seconds=21;

   loop {
    if  seconds <= 0{
        println!("Blastoff!");
        break;
    }
    if seconds % 2 == 0 {
        println!("{seconds} seconds (even number),skipping 3 seconds..");
        seconds -= 3;
        continue;
    }

    println!("{seconds} seconds to blastoff..");
    seconds -= 1;
   }



   ///////////////////////////////////////////////////////
   //While loop
   while seconds > 0  {
       if seconds % 2 == 0 {
           println!("{seconds} seconds (even number),skipping 3 seconds..");
           seconds -= 3;
           continue;
       }
       println!("{seconds} seconds to blastoff..");
       seconds -= 1;
   }



   ///////////////////////////////////////////////////////////
   //Recursion
   countdown(5);


}

fn even_or_odd(number:i32) {
    //below in rust we can also store the result of if else statement in a variable,but to impliment this there should be semicolon at the end of the else {} braces
   let result= if number%2==0 {
       "Even"
    }else {
        "odd"
    };

    println!("The number is {result}");
}

fn countdown(seconds:i32){
    if seconds == 0 {
        println!("Blastoff!");
    }
    else {
        println!("{seconds} seconds to blastoff.."); 
        countdown(seconds-1);
    }
}