use rand::Rng;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let _secret_number = generate_secret_number();
    Ok(())
}

fn generate_secret_number() -> u16 {
    let mut rng = rand::thread_rng();
    let mut array = [-1; 4];

    array[0] = rng.gen_range(1..10);
    let mut i: usize = 1;
    while i < 4 {
        let digit = rng.gen_range(0..10);
        if !array.contains(&digit) {
            array[i] = digit;
            i += 1;
        }
    }

    (array[0] * 1_000 + array[1] * 100 + array[2] * 10 + array[3]) as u16
}
