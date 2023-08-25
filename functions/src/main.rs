fn main() {
    println!("Hello, world!");

    another_function(five());

    println!("Start flow control - if:");
    flow_if(5);

    println!("Start flow control - match:");
    flow_match(5);

    println!("Start flow control - loop:");
    flow_loop();

    println!("Start flow control - while:");
    flow_while();

    println!("Start flow control - for:");
    flow_for();
}

// for rust you must specify the argument type
fn another_function(age: i32) {
    println!("You age is {}", age);
}

// define return type
fn five() -> i32 {
    5 // expression can return directly
}

// flow control if
fn flow_if(x: i32) {
    if x % 2 == 0 {
        println!("x is divisble by 2.")
    } else if x % 3 == 0 {
        println!("x is divisble by 3.")
    } else if x % 4 == 0 {
        println!("x is divisible by 4")
    } else {
        println!("x is not divisible by 2, 3, or 4")
    }
}

// flow control match
fn flow_match(x: i32) {
    match x % 2 {
        0 => println!("x is divisible by 2"),
        1 => println!("x is not divisible by 2"),
        _ => println!("x is not divisible by 2"),
    }
}

// flow control loop
fn flow_loop() {
    let mut x = 0;
    loop {
        x += 1;
        println!("{}", x);
        if x == 10 {
            break;
        }
    }
}

// flow control while
fn flow_while() {
    let mut x = 0;
    while x != 10 {
        println!("{}", x);
        x += 1;
    }
}

// flow control for
fn flow_for() {
    for x in 0..10 {
        println!("{}", x);
    }
}
