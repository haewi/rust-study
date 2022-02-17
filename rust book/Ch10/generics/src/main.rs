// generics in struct
struct Point<T> {
    x: T,
    y: T,
}

// declare <T> right after impl to tell Rust that T in Point<T> is a generic type not a concrete type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implement method for instances of Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        // return root of (x squared + y squared)
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// two differenct generics
struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {

    // no generic way
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);


    // generic way
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // generics with structs (both x and y should have the same type)
    let integer = Point {x: 5, y: 10}; // generic T is integer
    let float = Point {x: 1.0, y: 4.0}; // generic T is float

    // differenct types for x, y
    let int_and_float = Point2 {x: 5, y:4.0}; // no error

    let p = Point {x: 5, y:10};
    println!("p.x = {}", p.x());
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// function with generics
// "The functon largest is generic over some type T"
// "It has one parameter named list, which is a slice of values of type T"
// "The return value is type T"
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest { // error - for types that can't be compared
            largest = item;
        }
    }

    largest
}