use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 0 and 100!");
    println!("But remember, you only get 10 tries!");

    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..100);
    let mut tries = 10;

    loop {
        if tries == 0 {
            println!("You couldn't guess the number in 10 tries! Better luck next time.");
            return;
        }

        let mut guess = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number and a number only!");

        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Too small!");
                tries -= 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                tries -= 1;
            }
            Ordering::Equal => {
                println!("Correct number!");
                println!("Game over, you won!");
                return;
            }
        }

        println!("You guessed: {}", guess);
        println!("Tries left: {}", tries);
    }
}
