use bulls_and_cows::{run, Config};

fn main() {
    let config = Config::new();
    run(config).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
    })
}
