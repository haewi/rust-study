struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User { // the instance itself has to be mutable in order to change even one field
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("secondemail@example.com");

    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2name"),
        ..user1 // struct update syntax -> make the rest of the field same as user1
    };


    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black and origin are different types


    // unit-like structs [unit type is ()]

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // init shorthand syntax -> can be used when the field and the parameter name is the same
        username, // init shorthand syntax -> can be used when the field and the parameter name is the same
        active: true,
        sign_in_count: 1,
    }
}