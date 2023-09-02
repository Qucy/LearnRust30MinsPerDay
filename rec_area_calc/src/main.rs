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

// Below code demostates how to use enum in Rust
// enum implementation
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct implementation
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum implementation
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// enum implementation
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum implementation
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// enum implementation
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// enum implementation
// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// enum implementation
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct implementation
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// enum implementation
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// enum implementation
// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// enum implementation
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum implementation
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// enum implementation
// use std::io::Error;
// use std::io::Read;
// fn read_username_from_file() -> Result<String, Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// enum implementation
// use std::io::Error;
// use std::io::Read;
// fn read_username_from_file() -> Result<String, Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
