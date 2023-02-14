mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("from add_to_waitlist ...");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            // Absolute path
            crate::front_of_house::hosting::add_to_waitlist();
            // using Super
            super::hosting::add_to_waitlist();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//use crate::front_of_house::hosting;
use self::front_of_house::hosting;
use self::front_of_house::hosting as host;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    host::add_to_waitlist();
}

mod alt;
use crate::alt::hosting4;

pub fn eat_at_restaurant4() {
    hosting4::add_to_waitlist4();
}

mod libs {
    pub mod mylibs;
}

use libs::mylibs::hosting3;

pub fn eat_at_restaurant3() {
    hosting3::add_to_waitlist3();
}
