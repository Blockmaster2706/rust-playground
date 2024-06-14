use std::io;
use std::vec;
use colored::Colorize;
use modules::words::hide_word;

mod modules;

pub fn main() {
    loop {
        let current_word: String = modules::words::get_random_word();

        println!("The current word is: {}", current_word);

        let mut guessed = vec![];

        let mut current_error = String::new();

        let mut attempts = 0;

        loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            if attempts < 7 {
                println!("Remaining attempts: {}", 7-attempts);
                if current_error != "" {
                    println!("{}", current_error.red());
                    current_error.clear();
                }
            }
            else {
                print!("{}", (String::from("You have no more attempts left! The word was ").red()));
                print!("{}", current_word.red().underline());
                print!("{}", "\n".red());
                println!("Do you want to play again? (y/n)");

                let mut play_again = String::new();

                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Failed to read line");

                play_again = play_again.trim().to_string();

                if play_again != "y" {
                    println!("Thanks for playing!");
                    return;
                } else {
                    break;
                }
            }

            println!("You have already guessed: {}", guessed.join(", "));
            println!("Current word: {}", hide_word(&current_word, &guessed));

            let mut guess = String::new();

            println!("Please enter your guess: ");

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            guess = guess.trim().to_string();

            println!("You guessed: {}", guess);

            // Make sure guess is a single character
            if guess.chars().count() != 1 {
                current_error = "Please enter a single character".to_string();
                continue;
            }

            if guessed.contains(&guess) {
                current_error = "You already guessed that character".to_string();
                continue;
            }
            
            if !current_word.contains(&guess) {
                current_error = "You guessed incorrectly!".to_string();
                attempts += 1;
            }
            guessed.push(guess);

            if modules::words::is_word_guessed(&current_word, &guessed) {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                print!("{}", (String::from("You guessed the word! The word was ").green()));
                print!("{}", current_word.green().underline());
                print!("{}", "\n".green());

                println!("Do you want to play again? (y/n)");

                let mut play_again = String::new();

                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Failed to read line");

                play_again = play_again.trim().to_string();

                if play_again != "y" {
                    println!("Thanks for playing!");
                    return;
                } else {
                    break;
                }
            }
        }
    }
}
