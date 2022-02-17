// use std::fmt::Result;   // error
// use std::io::Result;    // error two items with the same name

use std::fmt::Result;               // no error
use std::io::Result as IoResult;    // new local name for the type

// clean large use lists
use std::cmp::Ordering;
use std::io;
// same as below
use std::{cmp::Ordering, io};

// another examnple
use std::io;
use std::io::Write;
// same as below
use std::io{self, Write};

// bring all public items defined in a path
use std::collections::*;

// mod: define a module
// modules can hold other modules, structs, functions etc.
mod front_of_house {
    // group functions related to hosting
    pub mod hosting { // make it public to parent module
        pub fn add_to_waitlist() { // make function also public to parent module
            println!("add_to_waitlist()");
        }

        fn seat_at_table() {
            println!("seat_at_table()");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("take_order()");
        }

        fn serve_order() {
            println!("serve_order()");
        }

        fn take_payment() {
            println!("take_payment()");
        }

        pub mod back_of_house {
            fn fix_incorrect_order() {
                println!("fix_incorrect_order()");
                cook_order();
                super::serve_order(); // super: keyword to start path at parent module
            }

            fn cook_order() {
                println!("cook_order()");
            }

            pub struct Breakfast {      // public struct
                pub toast: String,      // public toast field
                seasonal_fruit: String, // private seasonal_fruit field
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            pub enum Appetizer {
                Soup,
                Salad,
            }
        }
    }
}

// both absolute, relative paths are possible
// pub keyword for external code to use this path
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path (eat_at_restaurant and add_to_waitlist is in the same crate)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (front_of_house is defined at the same level of the module tree as eat_at_restaurant)
    front_of_house::hosting::add_to_waitlist();


    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);
    meal.toast = String::from("Wheat"); // no error
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // error seasonal_fruit field is private
    
    // valid, all variants of a public enum are public
    let order1 = front_of_house::serving::back_of_house::Appetizer::Soup;
    let order2 = front_of_house::serving::back_of_house::Appetizer::Salad;


    // valid, the use keyword made it possible
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}