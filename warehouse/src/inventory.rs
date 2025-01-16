mod inventory {
    const FLOOR_SPACE:i32 = 10000;

    //below we are writing pub as by default every element in this block is private,so if we want to access something from this module we have to use pub in front of the element we want to use outside
    pub const MANAGER:&str = "Ivan Inventory";

    #[derive(Debug)]
    enum ProductCategories {
        Ladder,
        Hammer,
    }

    struct Item {
        name:String,
        category:ProductCategories,
        quantity:u32,
    }

    fn talk_to_manager() {
        println!("Hey, {MANAGER}, how's your coffee?");
    }
}


