# Bulls and Cows
Realization of simple numerical version of the ***Bulls and Cows*** game written in ***Rust***

## Rules of the game
The program generates a secret number - 4-digit unsigned integer (digits are unique in this number).
The player must guess this number: player must input 4-digit number to guess it.
If the matching digits are in their right positions, they are "bulls", if in different positions, they are "cows".

For example:
- Secret number: 4271
- Opponent's try: 1234
- So the answer: 1 bull and 2 cows (the bull is "2", the cows are "4" and "1")

## How to use this program
There are 3 simple commands:
- `--help`            : shows all available options
- `--rules`           : shows the rules of the game
- `--play <attempts>` : play the game with given count of attempts (default is 4)

### Hope you will enjoy it!
