// Standard Input / Output Stream
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // mut = mutable
        let mut guess = String::new();

        // & indicates reference
        // Newline syntax is common for methods
        // .expect() does error handling
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadows variable "guess" into unsigned 32 bit int
        // trim() = strip()
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // Result is an enum that has the variants Ok or Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // goes to next iteration of loop
        };

        println!("You guessed: {}", guess);

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
