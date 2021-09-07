#[derive(Debug)] // make printing debugging info possible
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // impl : implementation block which defines methods
    fn area(&self) -> u32 { // methods can take ownership, borrow both immutably and mutable
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", area1(&rect1));

    println!("rect1 is {:?}", rect1); // :? means use 'Debug' as output format
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area() // method syntax: calls the method on our Rectagle instance
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(25);

    println!("sq is {:#?}", sq);
}

fn area1(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
