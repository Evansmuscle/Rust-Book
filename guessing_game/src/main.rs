use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 0 and 100!");
    println!("But remember, you only get 10 tries!");
    println!("You can always quit the game by just typing \"quit\".");

    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..100);
    let mut tries = 10;

    loop {
        if tries == 0 {
            println!("You couldn't guess the number in 10 tries! Better luck next time.");
            break;
        }

        let mut guess = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        if &guess == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

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
                break;
            }
        }

        println!("You guessed: {}", guess);
        println!("Tries left: {}", tries);
    }

    return;
}
