// mod [module name]; => syntax that tells Rust to load contents of the module fron another file with the same name as the module
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}