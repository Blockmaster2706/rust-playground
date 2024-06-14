use colored::Colorize;

pub fn run_prime_guesser() {
    let mut prime = 2;
    let mut primes = vec![];

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {

        if prime > 1000 {
            println!("You have reached the limit of this game.");
            println!("Press any key to return to the main menu.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            break;
        }

        if is_prime_number(&prime) {
            println!("What is the next prime?");
            
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
                println!("{} You have found {} prime numbers.", "Correct!".to_string().green(), primes.len());
            } else {
                println!("{} The next prime is {}.", "Incorrect!".to_string().red(), prime);
                println!("You have found {} prime numbers.", primes.len());
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