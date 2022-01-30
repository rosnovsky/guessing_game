use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut guess_counter = 1;
    loop {
        println!("Please input your guess #{}.", guess_counter);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                *&mut guess_counter += 1
            }
            Ordering::Greater => {
                println!("Too big!");
                *&mut guess_counter += 1;
            }
            Ordering::Equal => {
                println!("Exactly!");
                println!("Guessed in {} tries", *&mut guess_counter);
                break;
            }
        }
    }
}
