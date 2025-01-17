use fake::{Fake,Faker};

use warehouse::ProductCategories;

/// Create a fictonal Product
fn main() {
    //now even we empty the main.rs file now all these code will work 
    let random_category:ProductCategories = Faker.fake();
    println!("{:?}",random_category);
}