use std::error::Error;

use bulls_and_cows::{run, Config};

/// Entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    run(config)
}
