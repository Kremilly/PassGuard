mod score;
mod inputs;
mod password;
mod passphrase;

use inputs::Input;
use score::PassScore;
use password::PassGen;
use passphrase::PhraseGen;

fn main() {
    let method = Input::read_str("Choose a method (password/passphrase):");
    let show_score = Input::read_str("Show password strength? (y/n):");
    println!("---------------------------------");

    if method.to_lowercase() == "passphrase" {
        let min = Input::read_int("Enter the minimum word length:");
        let max = Input::read_int("Enter the maximum word length:");
        let words = Input::read_int("Enter the number of words in the passphrase:");
        let character = Input::read_str("Enter the character to separate the words:");
        let uppercase = Input::read_str("Include uppercase letters? (y/n):");
        let numbers = Input::read_str("Include numbers? (y/n):");
        
        let password = PhraseGen::new(
            min, max, words, character, uppercase, numbers
        ).get();

        PassScore::new(
            password, show_score
        ).get();
    } else if method.to_lowercase() == "password" {
        let size = Input::read_int("Enter the password length:");
        let uppercase = Input::read_str("Include uppercase letters? (y/n):");
        let numbers = Input::read_str("Include numbers? (y/n):");
        let symbols = Input::read_str("Include special characters? (y/n):");

        let password = PassGen::new(
            size, uppercase, numbers, symbols
        ).get();

        PassScore::new(
            password, show_score
        ).get();
    } else {
        println!("Invalid method");
    }
}
