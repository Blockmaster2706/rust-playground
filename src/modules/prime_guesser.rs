use colored::Colorize;
use std::collections::HashMap;



pub fn run_prime_guesser() {
    let mut prime = 2;
    let mut primes = vec![];
    let mut correctly_guessed = 0;

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let prime_hints: HashMap<u32, &str> = [
        (2, "What game number is this game?"),
        
        // Add more primes and their hints here
    ].iter().cloned().collect();

    loop {

        if prime > 1000 {
            println!("You have reached the limit of this game.");
            println!("Press any key to return to the main menu.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            break;
        }

        if is_prime_number(&prime) {
            if primes.len() > 0 {
                println!("What is the next prime after {}?", primes.last().unwrap());
            } else {
                println!("What is the next prime?");
            }
            match prime_hints.get(&prime) {
                Some(hint) => println!("Hint: {}", hint),
                None => {},
            }
            
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .unwrap();

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            
            if guess.contains("q") {
                println!("Thanks for playing!");
                break;
            }

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            if guess == prime {
                primes.push(prime);
                correctly_guessed += 1;
                println!("{} You have found {} prime numbers.", "Correct!".to_string().green(), correctly_guessed);
            } else {
                primes.push(prime);
                println!("{}, {} The next prime is {}.", "Incorrect!".to_string().red(), is_not_prime_number_because(&guess), prime);
                println!("You have found {} prime numbers.", correctly_guessed);
            }
            println!("The Prime Numbers so far were: {}", primes.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "));
        }

        prime += 1;
    }
}

fn is_prime_number(num: &u32) -> bool {
    if *num <= 1 {
        return false;
    }
    
    for p in 2..*num {
        if num % p == 0 {
            return false;
        }
    }
    true
}

fn is_not_prime_number_because(num: &u32) -> String {
    if *num <= 1 {
        return format!("because {} is less than 2.", num).to_string();
    }
    
    for p in 2..*num {
        if num % p == 0 {
            return format!("because {}'s lowest divider is {}.", num, p).to_string();
        }
    }
    return format!("because {} is a later prime number.", num).to_string();
}