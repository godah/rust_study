pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
            super::super_function();
        }

        pub fn seat_at_table() {
            println!("Seating at table");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Taking order");
        }

        pub fn serve_order() {
            println!("Serving order");
        }

        pub fn take_payment() {
            println!("Taking payment");
        }
    }

    fn super_function(){
        println!("This is a super function in front_of_house module");
    }
}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
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
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} {} toast please", meal.seasonal_fruit, meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad; 
}