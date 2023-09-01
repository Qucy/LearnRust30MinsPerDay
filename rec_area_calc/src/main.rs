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

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    // println!("rect is", rect); // will not compile due to missing trait `std::fmt::Display`
    println!("rect is {:?}", rect); // debug print
    println!("rect is {:#?}", rect); // pretty print
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
