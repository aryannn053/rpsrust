use std::io;
use rand::Rng;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(input: &str) -> Option<Choice> {
        match input.trim().to_lowercase().as_str() {
            "rock" => Some(Choice::Rock),
            "paper" => Some(Choice::Paper),
            "scissors" => Some(Choice::Scissors),
            _ => None,
        }
    }
}

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    loop {
        println!("Enter your choice (rock, paper, or scissors) or 'quit' to exit:");
        
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Failed to read line");

        let player_choice = match Choice::from_str(&player_input) {
            Some(choice) => choice,
            None => {
                println!("Invalid input. Please enter 'rock', 'paper', 'scissors', or 'quit'.");
                continue;
            }
        };

        if player_input.trim().to_lowercase() == "quit" {
            println!("Thanks for playing!");
            break;
        }

        let computer_choice = match rand::thread_rng().gen_range(0..=2) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            _ => Choice::Scissors,
        };

        println!("You chose: {:?}", player_choice);
        println!("Computer chose: {:?}", computer_choice);

        match (player_choice, computer_choice) {
            (Choice::Rock, Choice::Scissors) |
            (Choice::Paper, Choice::Rock) |
            (Choice::Scissors, Choice::Paper) => println!("You win!"),
            (Choice::Rock, Choice::Paper) |
            (Choice::Paper, Choice::Scissors) |
            (Choice::Scissors, Choice::Rock) => println!("Computer wins!"),
            _ => println!("It's a tie!"),
        }
    }
}
