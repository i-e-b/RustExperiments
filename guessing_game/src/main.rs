extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number, between 1 and 100");

    loop { // infinite loop without silly tricks :-)
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin() // from standard input,
            .read_line(&mut guess) // read up to newline, changing the guess value and returning io::Result
            .expect("Failed to read input"); // handle io::Result failure by crashing the program.

        let guess: u32 = match guess.trim().parse() { // shadow the old guess with a new numeric one
            Ok(num) => num,
            Err(_)  => {
                println!("Please enter a number");
                continue;
            }
        }; // the semicolon here is needed to end the assignment of `guess`

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}

