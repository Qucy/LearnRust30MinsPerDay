fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 7;
    println!("The value of x is {x}");

    // constant
    const MAX_POINTS: u32 = 100_000;
    println!("MAX POINTS is {}", MAX_POINTS);

    // shadow
    let y = 1;
    println!("Y is {}", y);
    let y = 2;
    println!("Y is {}", y);
    // shadow can change data type
    let y = "y";
    println!("Y is {}", y);
    // shadow have own scope
    {
        let y = "inner block";
        println!("Y is {}", y);
    }
    // shadow change back to previous value
    println!("Y is {}", y);

    // data type
    let unsigned_8: u8 = 1;
    let integer_32: i32 = 123;
    let single_float: f32 = 2.33;
    let double_float: f64 = 65.333;
    let b: bool = true;
    let charachter: char = 'c';

    println!(
        "{}, {}, {}, {}, {}, {}",
        unsigned_8, integer_32, single_float, double_float, b, charachter
    );

    // Tuple
    let tup: (i32, f64, u8) = (100, 64.4, 8);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b, c);

    // array
    let arr_a: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_b = [3; 5];
    println!("{}, {}", arr_a[0], arr_b[2]);

    // ======================= rust ownership =======================
    // string
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s); // will drop memory when out of scope

    // integer
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // push 2 value to stack

    // String
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); // error, s1 is moved to s2

    // clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // ======================= rust ownership =======================
    // function
    let s = String::from("Hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // return value
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    // ======================= rust ownership =======================
    // reference
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // mutable reference
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    // ======================= rust ownership =======================
    // slice
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    // ======================= rust ownership =======================
    // struct
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("user1"),
        email: String::from("32@qq.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.username);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
