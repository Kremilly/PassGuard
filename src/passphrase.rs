use std::char;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::score::PassScore;

pub struct PhraseGen {
    rng: ThreadRng,
    min: usize,
    max: usize,
    words: usize,
    character: String
}

impl PhraseGen {

    pub fn new(
        min: usize, max: usize, words: usize, character: String
    ) -> Self {
        let rng = rand::thread_rng();

        PhraseGen {
            rng,
            min,
            max,
            words,
            character
        }
    }

    pub fn generate_random_word(&mut self) -> String {
        let length = self.rng.gen_range(self.min..=self.max); // Usando range inclusivo
        let mut word = String::new();
        let mut has_number = false;

        for i in 0..length {
            let is_letter = self.rng.gen_bool(0.7);

            let character = if is_letter || (has_number && i > 0) {
                self.rng.gen_range(b'a'..=b'z') as char
            } else {
                has_number = true;
                self.rng.gen_range(b'0'..=b'9') as char
            };

            if i == 0 {
                word.push(character.to_ascii_uppercase());
            } else {
                word.push(character);
            }
        }

        word
    }

    pub fn generate_random_sentence(&mut self) -> String {
        let mut sentence = Vec::new();

        for _ in 0..self.words {
            let word = self.generate_random_word();
            sentence.push(word);
        }

        sentence.join(&self.character)
    }

    pub fn get(&mut self) {
        let password = self.generate_random_sentence();

        println!("---------------------------------");
        println!("Your password: {}", password);

        PassScore::new(&password).get();
    }

}
