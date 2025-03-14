use rand::Rng;
use colored::*;
use std::io::stdin;

pub fn guessing_number() {
    
    println!("{}", "--- Welcome to Guessing Game ---".blue());

    let secret_number = rand::thread_rng().gen_range(0..10);

    loop {
        println!("{}", "Enter your number a range between 0 to 10 ~> ".cyan());

        let guessing_number = stdin().lines().next()
        .unwrap().expect("Failed to read number!");

        let guessed_number = match guessing_number.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => return
        };

        if guessed_number == secret_number {
            println!("{}", "your guessing is correct.".green());
            break;
        } else {
            if secret_number < guessed_number {
                println!("{}", "your number is to big!".yellow());
            } else if secret_number > guessed_number {
                println!("{}", "your number is to small!".yellow());
            } else {
                println!("{}", "Your Guessing Is Not Correct.".red());
            }
        }

    }
}