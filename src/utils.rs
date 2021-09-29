//! This module has the utility functions for the game library

use rand::Rng;

/// Generates the 4-digit secret number (digits are unique in this number)
pub fn generate_secret_number() -> u16 {
    let mut rng = rand::thread_rng();

    let mut digits = [u8::MAX; 4];
    digits[0] = rng.gen_range(1..10);

    let mut i: usize = 1;
    while i < digits.len() {
        let digit = rng.gen_range(0..10);
        if !digits.contains(&digit) {
            digits[i] = digit;
            i += 1;
        }
    }

    from_digits(digits)
}

/// Returns the count of "cows" of the user provided number
///
/// # Arguments
///
/// * `secret_number` - secret number that was generated earlier
/// * `user_number` - number provided by user
pub fn cows(secret_number: u16, user_number: u16) -> u8 {
    let secret_number = to_digits(secret_number);
    let user_number = to_digits(user_number);

    let mut cows = 0;
    for (i, secret_digit) in secret_number.iter().enumerate() {
        let user_digit = user_number[i];
        if user_number.contains(secret_digit) && user_digit != *secret_digit {
            cows += 1
        }
    }
    cows
}

/// Returns the count of "bulls" of the user provided number
///
/// # Arguments
///
/// * `secret_number` - secret number that was generated earlier
/// * `user_number` - number provided by user
pub fn bulls(secret_number: u16, user_number: u16) -> u8 {
    let secret_number = to_digits(secret_number);
    let user_number = to_digits(user_number);

    let mut bulls = 0;
    for (i, secret_digit) in secret_number.iter().enumerate() {
        let user_digit = user_number[i];
        if user_digit == *secret_digit {
            bulls += 1
        }
    }
    bulls
}

/// Converts a number into an array of digits
///
/// # Arguments
///
/// * `number` - 4-digit number that will be converted
fn to_digits(number: u16) -> [u8; 4] {
    [
        (number / 1_000) as u8,
        ((number / 100) % 10) as u8,
        ((number / 10) % 10) as u8,
        (number % 10) as u8,
    ]
}

/// Converts a an array of digits into a number
///
/// # Arguments
///
/// * `digits` - array of digits that will be converted
fn from_digits(digits: [u8; 4]) -> u16 {
    (digits[0] as u16) * 1_000
        + (digits[1] as u16) * 100
        + (digits[2] as u16) * 10
        + digits[3] as u16
}

/// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_digits() {
        assert_eq!(1000, from_digits([1, 0, 0, 0]));
        assert_ne!(3742, from_digits([3, 6, 1, 9]));
    }

    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(1000), [1, 0, 0, 0]);
        assert_ne!(to_digits(3742), [3, 6, 1, 9]);
    }

    #[test]
    fn test_bulls() {
        assert_eq!(bulls(1234, 1234), 4);
        assert_eq!(bulls(3458, 3518), 2);
        assert_ne!(bulls(9876, 8273), 0);
        assert_ne!(bulls(3497, 3487), 4);
    }

    #[test]
    fn test_cows() {
        assert_eq!(cows(1234, 5678), 0);
        assert_eq!(cows(9358, 5893), 4);
        assert_ne!(cows(1285, 9371), 2);
        assert_ne!(cows(3950, 5092), 4);
    }
}
