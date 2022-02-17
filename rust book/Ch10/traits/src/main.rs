// the project name is 'traits'
// Summary trait has to be imported also because items from traits can only be used if the trait is in scope
use traits::{Summary, Tweet}; // import Summary trait and Tweet struct
use traits::{Summary2, NewsArticle2};

// generics in struct
struct Point<T> {
    x: T,
    y: T,
}

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle2 {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    // generic way
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// can return type that implements Summary trait
// however impl Trait syntax allows only one type to be returned (can't return A type or B type even though they both implement the Summary trait)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// > operator is defined on the trait std::cmp::PartialOrd
// generic T should implement PartialOrd (its in prelude)
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list { // item is type T
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// another solution
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // reference of the element in list

    for item in list { // item is type &(T)
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

// the type Pair<T> always implements new()
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}


// the type Pair<T> only implements cmp_display() when T implements the Display and PartialOrd trait
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}