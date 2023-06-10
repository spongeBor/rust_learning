mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        mod back_of_house {
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

            fn cook_order() {}
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order()
            }
        }
        pub fn eat_at_restaurant() {
          let mut meal = back_of_house::Breakfast::summer("Rye");
          meal.toast = String::from("Wheat");
          println!("I'd like {} toast please", meal.toast);
          // meal.seasonal_fruit = String::from("rewrewr"); // 私有不能

        }
    }
}

use front_of_house::hosting;
pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // re path
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist()
}
