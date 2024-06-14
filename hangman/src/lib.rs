use std::io;

mod modules;

pub fn main() {
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        modules::greeting::greet();
        
        println!("What do you want to play?");

        println!("1. Hangman");

        println!("2. Quit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => {
                modules::hangman::run_hangman();
            },
            "2" => {
                println!("Thanks for playing!");
                break;
            },
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
