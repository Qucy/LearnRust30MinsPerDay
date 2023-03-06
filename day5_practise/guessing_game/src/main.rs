use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!!!");

    println!("Please input your guess.");

    let _secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {_secret_number}"); -- only for debug purpose

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(num) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }

}
