#![warn(clippy::all, clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut guess_counter: i32 = 0;

    loop {
        guess_counter = guess_counter_increment(guess_counter);
        println!("Please input your guess #{:#?}.", guess_counter);

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("Exactly!");
                println!("Guessed in {} tries", guess_counter);
                break;
            }
        }
    }
}

fn guess_counter_increment(i: i32) -> i32 {
    i + 1
}

#[cfg(test)]
mod useless_guess_test {

    #[test]
    fn useless_guess_works() {
        let secret_number = 50;
        let guessed_number = 50;
        assert!(guessed_number == secret_number);
    }
}
