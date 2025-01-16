//////////////////////////////////////////////////////////////////////////////////////////
//Generics
//A Generic is a type argument.It's a placeholder for future type
//a generic is like a parameter,but for a type like i32 or string or bool or many more

#[derive(Debug)]

struct DeliSandwich {}

#[derive(Debug)]
struct TreasureChest<T> {
    captain:String,
    treasure:T,
}

//so the methods below will only exist to those intsance of structs where the type of <> whatever in it ,like in our case String for other those methods will not work
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure= self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) ->usize {
        self.treasure.len()
    }
}

//so after applying <> this after impl we can also now define the generic methods ,like methods which will work on any type
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

//Generics in Enums
enum Cheesesteak<T> {
    Plane,
    Toppings(T),
}

fn main() {
    println!("{}",identity(5.8));
    println!("{}",identity(5));
    println!("{}",identity("types"));
    println!("{}",identity(String::from("string")));
    println!("{}",identity(true));

    //so even custom type is supported by generic identity function
    println!("{:?}",identity(DeliSandwich {}));



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The Turbofish Operator
    //It is a (::<typename>) ,eg->::<i32> , ::<&str> ,etc.
    //the advantage is that now we can customize this if we want a different type
    //like we can define accurate type for that value as rust will always assign i32 for integer ,but we can also give i8,i16 and all
    println!("{}",identity::<i32>(10));
    println!("{}",identity::<&str>("string"));
    println!("{:?}",identity::<DeliSandwich>(DeliSandwich {}));


    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Multiple Generics
    make_tuple("hello", 5);

    //so if both paramter in function below is taking type as T like generic ,then both should be of same type ,and type checking will be from fisrt parameter as if first paramter is string then second should also be string
    make_tuple1("Hello", "World");

    //so now in below function we can provide the both parameter with different types
    make_tupe2("Hello", 2);
    make_tupe2(true, 9.7);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Generics in Structs
    let gold_chest=TreasureChest {
        captain:String::from("Jack Sparow"),
        treasure:"Gold",
    };
    println!("{:?}",gold_chest);

    let silver_chest=TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:String::from("Silver"),
    };
    println!("{:?}",silver_chest);

    let special_chest  = TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:["Gold","Silver","Platinum"],
    };
    println!("{:?}",special_chest);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Generics and impl Blocks I
    let mut  silver_chest1=TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:String::from("            Silver                 "),
    };
    silver_chest1.clean_treasure();
    println!("{:?}",silver_chest1);

    let special_chest1  = TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:["Gold","Silver","Platinum"],
    };
    println!("{}", special_chest1.amount_of_treasure());



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Generics and impl Blocks II
    let gold_chest2=TreasureChest {
        captain:String::from("Jack Sparow"),
        treasure:"Gold",
    };
    println!("{:?}",gold_chest2.capital_captain());

    let silver_chest2=TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:String::from("Silver"),
    };
    println!("{:?}",silver_chest2.capital_captain());

    let special_chest2  = TreasureChest {
        captain:String::from("Bloodsail"),
        treasure:["Gold","Silver","Platinum"],
    };
    println!("{:?}",special_chest2.capital_captain());




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Generics in Enums
    let mushroom= Cheesesteak::Toppings("mushroom");
    let onions=Cheesesteak::Toppings("onion".to_string());
    let topping="bacon".to_string();
    let bacon=Cheesesteak::Toppings(&topping);

    //as for below code be have to provide some type as plane variant in enum above ,does not have any perticular type but the enum need some type peramater
    let mut plain:Cheesesteak<String>=Cheesesteak::Plane;
    plain = Cheesesteak::Toppings("sausage".to_string());
}

//below is a generic function,here we dont know the specific type of value we are passing but that should be a valid type ,so it is used as generic 
fn identity<T>(value:T) ->T {
    value
}

fn make_tuple<T>(first:T,second:i32) ->(T,i32) {
    (first,second)
}

//so here below we can provide paramter to both but both paramter should be of same type in below case
fn make_tuple1<T>(first:T,second:T) ->(T,T) {
    (first,second)
}

//Multiple generic function
fn make_tupe2<T, U>(fisrt:T,second:U) -> (T,U) {
    (fisrt,second)
}

///////////////////////////////////////////
//funtion which takes only specific type and return only that type
fn identity_i32(value:i32) -> i32 {
    value
}

fn identity_bool(value:bool) ->bool {
    value
}