use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input the guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mul guess);
        .expect("Failed to read line");

    println("You guessed: {}", guess);

}