use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number (1-100)!");

    let correct_number = rand::thread_rng().gen_range(1, 100);
    //println!("The correct number is: {}", correct_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&correct_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "Correct! You win!".green());
                break;
            }
        }
    }
}