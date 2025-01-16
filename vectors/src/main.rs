//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Vectors
//It is a data structure that is similar to an array

fn main() {
    // let pizza_diameters: Vec<i32>= Vec::new();
    //alternative of above
    let pizza_diameter = Vec::<i32>::new();
    println!("{pizza_diameter:?}");

    let pastas = Vec::<&str>::new();
    println!("{pastas:?}");

    //vec![]
    let pizza_diameter1 = vec![8, 10, 12, 14];
    println!("{pizza_diameter1:?}");




    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Adding and Removing Elements
    let mut pizza_diameter2 = vec![8, 10, 12, 14];
    pizza_diameter2.push(16);
    pizza_diameter2.push(18);

    pizza_diameter2.insert(0, 4);

    //the 'pop' method remove the last element from the vector and return it
    let last_pizza_diameter = pizza_diameter2.pop();
    println!("{:?}", last_pizza_diameter);

    let third_diameter_from_start = pizza_diameter2.remove(2);
    println!("{}", third_diameter_from_start);

    println!("{pizza_diameter2:?}");




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Reading Vector Elements
    let diamters = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    //so below all the ownerships of variable used in vactor is moves to vector
    let pizza_toppings = vec![pepperoni,mushroom,sausage];
    
    //so in below code the ownership does not move the diameters vector is still the owner, as the 'diamter' vector has i32 types elements, so those element impliment copy trait
    let value = diamters[2];

    //this will not work as String does not impliment the copy trait
    // let value1 = pizza_toppings[2];
    let reference = &pizza_toppings[2];
    println!("{reference}");

    let pizza_slice = &diamters[1..3];
    println!("{pizza_slice:?}");



    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The get Method
    //The get method extracts a vector element by index posistion.it returns an Option enum

    let option = pizza_toppings.get(2);

    match option {
        Some(toppings) => println!("The topping is {toppings}"),
        None => println!("No value at that index position"),
    }




    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Ownership with Vectors
    let mut delicious_toppings = pizza_toppings;

    let topping_reference = &delicious_toppings[1];
    println!("The toppings is {}" ,topping_reference);

    delicious_toppings.push(String::from("Olives"));

    // println!("The toppings is {}" ,topping_reference);




    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Writing Vector Elements
    let pepperoni1 = String::from("Pepperoni");
    let mushroom1 = String::from("Mushroom");
    let sausage1 = String::from("Sausage");

    let mut pizza_toppings1 = vec![pepperoni1,mushroom1,sausage1];

    pizza_toppings1[1] = String::from("Olives");
    println!("{pizza_toppings1:#?}");

    let target_topping = &mut pizza_toppings1[2];
    target_topping.push_str(" and Meatballs");

    let another_topping = &pizza_toppings1[2];

    println!("{another_topping}");
    println!("{pizza_toppings1:#?}");



    ////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Vector Capacity Behind the Scenes
    //The vector capacity is the maximum number of elements that the vector can contain
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!("Length: {}, Capacity: {}", seasons.len(),seasons.capacity());

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Length: {}, Capacity: {}", seasons.len(),seasons.capacity());

    seasons.push("Summer");
    println!("Length: {}, Capacity: {}", seasons.len(),seasons.capacity());
}
