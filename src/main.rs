mod score;
mod inputs;
mod passphrase;

use inputs::Inputs;
use passphrase::PhraseGen;

fn main() {
    let min = Inputs::read_int("Enter the minimum word length:");
    let max = Inputs::read_int("Enter the maximum word length:");
    let words = Inputs::read_int("Enter the number of words in the passphrase:");
    let character = Inputs::read_str("Enter the character to separate the words:");
    
    PhraseGen::new(
        min, max, words, character
    ).get();
}
