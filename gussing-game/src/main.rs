extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, Naim!");
    println!("Guess a number...");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to guess number");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Number is too small"); }
            Ordering::Greater => { println!("Number is too large"); }
            Ordering::Equal => {
                println!("The secret number is {}", secret_number);
                println!("You WIN!");
                break;
            }
        }
    }
}
