use std::io;

fn main() {
    println!("Guess the number between 0 and 100!");
    println!("Please input your guess:");

    const CAPACITY: usize = 101;
    let mut guess = String::with_capacity(CAPACITY);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {}", guess);
}
