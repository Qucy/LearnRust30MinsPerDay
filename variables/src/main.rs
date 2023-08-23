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
}
