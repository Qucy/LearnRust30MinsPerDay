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
}
