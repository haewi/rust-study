use std::fmt::{Display, Debug};

pub trait Summary { // declare a trait
    // method signature (describe the behaviors of the types that implement this trait)
    fn summarize(&self) -> String; // the method body is provided by the types
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait on NewsArticle type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implementing a trait on Tweet type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary2 {
    fn summarize(&self) -> String{
        String::from("(Read more...)") // default behavior
    }
}

pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait on NewsArticle type
impl Summary2 for NewsArticle2 {
    // empty block of impl to use the default behavior
    // if overriding, the syntax is the same as methods with no default behavior
}

// impl Trait syntax - item is some type that implements Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize()); // can call methods in Summary trait
}

// trait bound syntax - same as [pub fn notify(item: &impl Summary)]
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: &(impl Summary + Display)) {} // item must implement both Summary adn Display
pub fn notify3_tb<T: Summary + Display>(item: &T) {} // same as above

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u:&U) -> i32 {5} // hard to read
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone, // where clause to specify trait bounds
          U: Clone + Debug
{5} // easier to read






