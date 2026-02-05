use std::io::{self, Write};
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess > secret {
        1
    } else if guess < secret {
        -1
    } else {
        0
    }
}

fn main() {
    let mut secret = 4;
    let mut guess = String::new();
    let mut number: i32;
    let mut attempts = 0;

    print!("Take a guess: ");
    io::stdout().flush().unwrap();
    
    loop {
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        number = guess.trim().parse().expect("Please enter a valid number.");
        
        attempts += 1;
        if check_guess(number, secret) == 0 {
            println!("Congratulations you guessed right!");
            println!("It took you this many guesses: {}", attempts);
            break;
        } else if check_guess(number, secret) == 1 {
            println!("Sorry, your guess is too high.");
        } else {
            println!("Sorry, your guess is too low.");
        }

        print!("Guess again: ");
        io::stdout().flush().unwrap();
    }
}