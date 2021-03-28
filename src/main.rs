fn main() {
    bulls_and_cows::run().unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
    })
}
