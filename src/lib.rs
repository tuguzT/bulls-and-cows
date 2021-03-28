mod utils;

use std::io::Write;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello there!\n\
        You're playing in the 'Bulls and Cows' game!\n\
        Let me tell you the rules:\n\
        \t- computer generates a 4-digit positive number (all the digits are different),\n\
        \t- and YOU, the player, must guess all numbers in it!\n\
        If the matching digits are in their right positions, they are called 'bulls', \
        but if in different positions, they are 'cows'.\n\
        One important note: you have only 4 attempts to guess the number!");
    let secret_number = utils::generate_secret_number();

    let mut attempts_remaining: u8 = 4;
    let mut user_input = String::new();
    while attempts_remaining > 0 {
        print!("\n{} attempts remaining. Enter your number: ", attempts_remaining);
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut user_input)?;
        let user_number = user_input.trim().parse::<u16>();
        match user_number {
            Ok(user_number) => {
                if user_number < 1_000 {
                    println!("Your number is too small! It must be 4-digit!");
                } else if user_number > 9_999 {
                    println!("Your number is too big! It must be 4-digit!");
                } else {
                    println!("Your number is: {}", user_number);
                    let _cows = utils::cows(secret_number, user_number);
                    let _bulls = utils::bulls(secret_number, user_number);
                    attempts_remaining -= 1
                }
            }
            Err(_) => {
                println!("Your input is not a positive 4-digit number!");
            }
        };
        user_input.clear();
    }

    println!("\nSorry, but you have lost: you haven't guess a secret number!");
    Ok(())
}
