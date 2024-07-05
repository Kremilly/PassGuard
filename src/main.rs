mod passphrase;
use passphrase::PhraseGen;

fn main() {
    let min = 3;
    let max = 8;
    let num_words = 5;
    let mut passphrase = PhraseGen::new(min, max, num_words);

    let sentence = passphrase.generate_random_sentence();
    println!("Generated sentence: {}", sentence);
}
