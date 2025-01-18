//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Hashmap
//A hash map is a collection type that consists of key-value pairs

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    //below the first element in hashmap is key and every key should be different from another
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99 );
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);

    println!("{:?}",menu);

    // let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    //we can also use turbofish operator here to define the types
    let mut country_capitals = HashMap::<&str, &str>::new();

    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");

    println!("{:?}",country_capitals);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The remove Method
    //the remove method deletes a key-value by using the key

    let data = [
        ("Bobby",7),
        ("Grant",4),
        ("Ben",6),
    ];

    //here in below code the 'data' will get converted to equivalent hashmap , by using ::from()
    let mut years_at_company = HashMap::from(data);
    println!("{:?}",years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}",ben);
    println!("{}",ben.unwrap());
    println!("{:?}",years_at_company);



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Hash Maps and Ownership
    let mut coffee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    //so below the ownership of the 'drink' variable and 'milk' variable are moved to 'coffee_parings' variable
    // coffee_pairings.insert(drink, milk);

    //now to not move the ownership we can do this
    coffee_pairings.insert(&drink, &milk);

    println!("{:?}",coffee_pairings);
    println!("{}",coffee_pairings.len());
    println!("{drink} {milk}");



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Access a Value by Key
    let mut coffee_pairings1: HashMap<&str, &str> = HashMap::new();
    let drink1 = String::from("Latte");
    let milk1 = String::from("Oat Milk");

    coffee_pairings.insert(&drink1, &milk1);
    coffee_pairings1.insert("Flat White", "Almond Milk");

    //below code will give the key of a data in hashmap which we mentioned in []
    //but in below code if the key does not exist then it will cause panic/error
    // let value = coffee_pairings1["Flat White"];

    //alternative
    // let value = coffee_pairings1.get("Flat White");

    //now best option
    let value = coffee_pairings1
    .get("Cappuccino")
    .copied()
    .unwrap_or("Unknown Milk");

    println!("{}",value);




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //Overwriting a Value with an Existing Key
    let mut coffee_pairings2: HashMap<&str, &str> = HashMap::new();
    let drink2 = String::from("Latte");
    let milk2 = String::from("Oat Milk");

    coffee_pairings2.insert(&drink2, &milk2);
    coffee_pairings2.insert("Flat White", "Almond Milk");
    println!("{:?}",coffee_pairings2);

    //below code will replace above mentioned same key value with below key value as the key here is same as above mentioned one
    coffee_pairings2.insert("Latte", "Pistachio Milk");
    println!("{:?}",coffee_pairings2);



    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The entry Method
    //The entry method accepts a hash map key and it returns an enum called 'Entry' and that enum has two variants representing the possibility that the given key exists and the possibility that the given key does not exist

    let mut coffee_pairings3: HashMap<&str, &str> = HashMap::new();
    let drink3 = String::from("Latte");
    let milk3 = String::from("Oat Milk");

    coffee_pairings3.insert(&drink3, &milk3);
    coffee_pairings3.insert("Flat White", "Almond Milk");

    //so below the entry method returns that 'Entry' enum ,and it is the enum and it is the enum itself that has a method called 'or_insert'
    //'or_insert' will add a key value pair to the HashMap ,but only if the key does not exist
    coffee_pairings3.entry("Latte").or_insert("Pistacho Milk");
    println!("{:?}",coffee_pairings3);

    coffee_pairings3.entry("Cappuccino").or_insert("Pistacho Milk");
    println!("{:?}",coffee_pairings3);




    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //The HashSet
    //A hashset is a collection type that stores unique values.
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}",concert_queue);

    concert_queue.insert("Molley");
    concert_queue.insert("Megan");

    println!("{:?}",concert_queue);

    //below trying to insert the element that already exist,so here hashset will just reject the entry as it already exists
    concert_queue.insert("Molley");
    println!("{:?}",concert_queue);

    //in below code it will return true or false ,it the entry removed then true and if not then false
    println!("{}",concert_queue.remove("Megan"));
    println!("{}",concert_queue.remove("Franny"));
    println!("{:?}",concert_queue);

    println!("{}",concert_queue.contains("Molley"));
    println!("{}",concert_queue.contains("Fred"));

    println!("{:?}",concert_queue.get("Molley"));




    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //HashSet Operations
    let mut concert_queue1:HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue1.insert("Boris");
    concert_queue1.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    //below union will give all the entries that are found across both my 'concert_queue1' hashset and my 'movie_queue' hashset as well
    println!("{:?}",concert_queue1.union(&movie_queue));
    println!("{:?}",movie_queue.union(&concert_queue1));

    //below difference method is going  to give the values that are found in the first set, which is the one that the method is invoked upon but not found in the second set
    println!("{:?}",concert_queue1.difference(&movie_queue));
    println!("{:?}",movie_queue.difference(&concert_queue1));

    //below symetric_difference is going to give the values that are in either one of the sets but not both
    println!("{:?}",concert_queue1.symmetric_difference(&movie_queue));
    println!("{:?}",movie_queue.symmetric_difference(&concert_queue1));

    //this method return true if the sets have no value in common
    println!("{:?}",concert_queue1.is_disjoint(&movie_queue));
    println!("{:?}",movie_queue.is_disjoint(&concert_queue1));

    //below 'is_subset' is going to return true if the set that the method is invoked upon is a subset of the argument set 
    println!("{:?}",concert_queue1.is_subset(&movie_queue));
    // println!("{:?}",movie_queue.difference(&concert_queue1));

}


