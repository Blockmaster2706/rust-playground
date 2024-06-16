pub mod states;

use std::io;

use rand::prelude::SliceRandom;

pub fn run_tictactoe() {
    loop {
        let mut board = ["1".to_string(), "2".to_string(), "3".to_string(),
                        "4".to_string(), "5".to_string(), "6".to_string(),
                        "7".to_string(), "8".to_string(), "9".to_string()];

        loop {
            states::render_state(&board);

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            if choice.trim() == "q" {
                println!("Thanks for playing!");
                return;
            }

            let choice: usize = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid choice. Please try again.");
                    continue;
                }
            };

            if choice < 1 || choice > 9 {
                println!("Invalid choice. Please try again.");
                continue;
            }

            if board[choice - 1] == "X" || board[choice - 1] == "O" {
                println!("Invalid choice. Please try again.");
                continue;
            }

            board[choice - 1] = "X".to_string();

            if check_win(&board, "X") {
                states::render_state(&board);
                println!("You win!");
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

            let mut available_choices = vec![];

            for (i, cell) in board.iter().enumerate() {
                if cell != "X" && cell != "O" {
                    available_choices.push(i);
                }
            }

            if available_choices.is_empty() {
                states::render_state(&board);
                println!("It's a draw!");
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

            let computer_choice = available_choices.choose_mut(&mut rand::thread_rng()).unwrap();
            board[*computer_choice] = "O".to_string();

            if check_win(&board, "O") {
                states::render_state(&board);
                println!("You lose!");
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

fn check_win(board: &[String; 9], player: &str) -> bool {
    let winning_combinations = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6]
    ];

    for combination in winning_combinations {
        if combination.iter().all(|&i| board[i] == player) {
            return true;
        }
    }

    false
}