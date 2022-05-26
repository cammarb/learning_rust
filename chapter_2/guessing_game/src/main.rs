use std::io; //io - input/output library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // mut makes the variable mutable.
    // The :: syntax indicates that new is an associated function of String
    let mut guess = String::new();
    io::stdin()
        // .read_line() method of the stdin library
        // The & indicates that this argument is a reference
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
