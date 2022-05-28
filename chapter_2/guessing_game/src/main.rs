use rand::Rng; // defines methods that random number generators implement
use std::cmp::Ordering;
use std::io; //io - input/output library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // Line just for testing
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // mut makes the variable mutable.
        // The :: syntax indicates that new is an associated function of String
        let mut guess = String::new();
        io::stdin()
            // .read_line() method of the stdin library
            // The & indicates that this argument is a reference
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        // NOTE: crate is a collection of Rust source code files
        // The cmp method compares two values and can be called on anything that can be compared
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
