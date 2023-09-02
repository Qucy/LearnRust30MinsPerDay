// Below code demostates how to use struct and method in Rust

// This program calculates the area of a rectangle. (normal version)
// fn main() {
//     let w = 30;
//     let h = 50;

//     println!("The area of the rectangle is {} square pixels.", area(w, h));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// This program calculates the area of a rectangle. (tuple version)
// fn main() {
//     let rect = (30, 50);

//     println!("The area of the rectangle is {} square pixels.", area(rect));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// This program calculates the area of a rectangle. (struct version)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct implementation
impl Rectangle {
    // method implementation
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method implementation
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function implementation
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

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(3);

    // println!("rect is", rect); // will not compile due to missing trait `std::fmt::Display`
    println!("rect is {:?}", rect1); // debug print
    println!("rect is {:#?}", rect1); // pretty print

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));

    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));

    println!("square is {:?}", square);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
