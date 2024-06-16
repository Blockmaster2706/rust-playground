mod states;

use std::io;
use rand::seq::SliceRandom;

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

            let available_choices: Vec<usize> = board
                .iter()
                .enumerate()
                .filter(|(_, cell)| **cell != "X" && **cell != "O")
                .map(|(i, _)| i)
                .collect();

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

            let computer_choice = choose_computer_move(&board);
            board[computer_choice] = "O".to_string();

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

fn choose_computer_move(board: &[String; 9]) -> usize {
    // First, check if there is a winning move for the computer
    for i in 0..9 {
        if board[i] != "X" && board[i] != "O" {
            let mut new_board = board.clone();
            new_board[i] = "O".to_string();
            if check_win(&new_board, "O") {
                return i;
            }
        }
    }

    // Then, check if there is a winning move for the player and block it
    for i in 0..9 {
        if board[i] != "X" && board[i] != "O" {
            let mut new_board = board.clone();
            new_board[i] = "X".to_string();
            if check_win(&new_board, "X") {
                return i;
            }
        }
    }

    // If no winning moves are available, choose a random available move
    let available_choices: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|(_, cell)| **cell != "X" && **cell != "O")
        .map(|(i, _)| i)
        .collect();

    *available_choices.choose(&mut rand::thread_rng()).unwrap()
}

fn check_win(board: &[String; 9], player: &str) -> bool {
    for i in 0..3 {
        if board[i] == player && board[i+3] == player && board[i+6] == player {
            return true;
        }
        if board[i*3] == player && board[i*3+1] == player && board[i*3+2] == player {
            return true;
        }
    }

    if board[0] == player && board[4] == player && board[8] == player {
        return true;
    }

    if board[2] == player && board[4] == player && board[6] == player {
        return true;
    }

    false
}