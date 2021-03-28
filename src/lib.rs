mod utils;

use std::io::Write;
use std::error::Error;

pub struct Config {
    commands: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut commands: Vec<String> = std::env::args().collect();
        commands.remove(0);

        Self {
            commands,
        }
    }
}

const RULES: &'static str = "Let me tell you the rules:\n\
        \t- computer generates a 4-digit positive number (all the digits are different),\n\
        \t- and YOU, the player, must guess it!\n\
        If the matching digits are in their right positions, they are called 'bulls', \
        but if in different positions, they are 'cows'.\n";

const HELP: &'static str = "'Bulls and Cows' game\n\
        Usage: bulls_and_cows <options>\n\
        Available options:\n\
        \t--play <attempts> : play the game with given count of attempts (default is 4)\n\
        \t--help            : shows all available options\n\
        \t--rules           : shows the rules of the game";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.commands.is_empty() || config.commands[0] == "--help" {
        println!("{}", HELP);
    } else if config.commands[0] == "--play" {
        let attempts_raw = config.commands.get(1);
        let attempts: u8 = match attempts_raw {
            None => 4,
            Some(attempts_raw) => match attempts_raw.parse::<u8>() {
                Ok(attempts) => if attempts >= 4 {
                    attempts
                } else {
                    eprintln!("Count of attempts mustn't be less than 4!");
                    return Ok(())
                },
                Err(_) => {
                    eprintln!("Count of attempts must be positive number!");
                    return Ok(())
                }
            }
        };
        return play(attempts)
    } else if config.commands[0] == "--rules" {
        println!("{}", RULES);
    } else {
        eprintln!("Wrong option was given! Use --help option to see available options.");
    }
    Ok(())
}

fn play(attempts: u8) -> Result<(), Box<dyn Error>> {
    let mut attempts_remaining = attempts;
    println!(
        "Hello there!\n\
        You're playing in the 'Bulls and Cows' game!\n{}\
        One important note: you have only {} attempts to guess the number!",
        RULES, attempts
    );
    let secret_number = utils::generate_secret_number();

    let mut user_input = String::new();
    while attempts_remaining > 0 {
        print!("\n{} attempts remaining. Enter your number: ", attempts_remaining);
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut user_input)?;
        let user_number = user_input.trim().parse::<u16>();
        match user_number {
            Ok(user_number) => {
                if user_number < 1_000 {
                    eprintln!("Your number is too small! It must be 4-digit!");
                } else if user_number > 9_999 {
                    eprintln!("Your number is too big! It must be 4-digit!");
                } else {
                    let cows = utils::cows(secret_number, user_number);
                    let bulls = utils::bulls(secret_number, user_number);
                    if bulls == 4 {
                        println!("CONGRATULATIONS!!! You have guessed the secret number!");
                        return Ok(())
                    }
                    println!("{} cows and {} bulls for now!", cows, bulls);
                    attempts_remaining -= 1;
                }
            }
            Err(_) => {
                eprintln!("Your input is not a positive 4-digit number!");
            }
        };
        user_input.clear();
    }

    println!("\nSorry, but you have lost: you haven't guess a secret number!");
    Ok(())
}
