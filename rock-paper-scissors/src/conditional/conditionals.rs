use std::io::stdin;
use rand::seq::SliceRandom;

pub fn conditional_input () {

    let mut user_score = 0;
    let mut computer_score = 0;

    loop {

        if (user_score == 3) | (computer_score == 3){
            println!("-- Do you finish the game or start over? ---");
            println!("[yes, no] ~> ");
            let game = stdin().lines().next().unwrap().expect("Input is invalid").to_lowercase();

            if game == "yes" {
                println!("--- Game Over ---");
                break;
            } else if game == "no" {
                user_score = 0;
                computer_score = 0;
            } else {
                println!("Input is invalid");
                continue;
            }
        }
        println!("Please select one of the moves to start the game:");
        println!("[rock, paper, scissors] or [exist] ~> ");
        
        let user_input = stdin().lines().next().unwrap().expect("Input is invalid").to_lowercase();

        if user_input == "exist" {
            println!("--- Exist Game ---");
            break;
        };

        let computer_choices = ["rock", "paper", "scissors"];
        let computer_input = computer_choices.choose(&mut rand::thread_rng());
    
        match computer_input{
            None => {
                println!("None");
            }
            Some(computer_input) => {
                conditional_game(&user_input, &computer_input, &mut user_score, &mut computer_score);
                println!("{user_score} vs {computer_score}");
            }
        }   
    }
}

pub fn conditional_game(user_input: &str, computer_input: &str, user_score: &mut i32, computer_score: &mut i32) {
    
    if user_input == "rock" {

        if computer_input == "rock" {
            println!("{user_input}-{computer_input} : Equal");
            *user_score += 1;
            *computer_score += 1;

        } else if computer_input == "paper" {
            println!("{user_input}-{computer_input} : Loss");
            *computer_score += 1;

        } else if computer_input == "scissors" {
            println!("{user_input}-{computer_input} : Win");
            *user_score += 1;
        }
        
    } else if user_input == "paper" {

        if computer_input == "rock" {
           println!("{user_input}-{computer_input} : Win");
            *user_score += 1;

        } else if computer_input == "paper" {
            println!("{user_input}-{computer_input} : Equal");
            *user_score += 1;
            *computer_score += 1;

        } else if computer_input == "scissors" {
            println!("{user_input}-{computer_input} : Loss");
            *computer_score += 1;
        }
        
    } else if user_input == "scissors" {

        if computer_input == "rock" {
            println!("{user_input}-{computer_input} : Loss");
            *computer_score += 1;

        } else if computer_input == "paper" {
            println!("{user_input}-{computer_input} : Win");
            *user_score += 1;

        } else if computer_input == "scissors" {
            println!("{user_input}-{computer_input} : Equal");
            *user_score += 1;
            *computer_score += 1;

        }
    } else {
        println!("input is invalid")
    }

}
