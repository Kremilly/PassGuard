use rand::Rng;
use rand::rngs::ThreadRng;

pub struct PhraseGen {
    rng: ThreadRng,
    min: usize,
    max: usize,
    num_words: usize,
}

impl PhraseGen {

    pub fn new(min: usize, max: usize, num_words: usize) -> Self {
        let rng = rand::thread_rng();
        
        PhraseGen {
            rng, 
            min, 
            max, 
            num_words
        }
    }

    pub fn generate_random_word(&mut self) -> String {
        let length = self.rng.gen_range(self.min..self.max);
        let mut word = String::new();
    
        for _ in 0..length {
            let is_letter = self.rng.gen_bool(0.7);
            
            let character = if is_letter {
                self.rng.gen_range(b'a'..=b'z') as char
            } else {
                self.rng.gen_range(b'0'..=b'9') as char
            };
            
            word.push(character);
        }
    
        word
    }
    
    pub fn generate_random_sentence(&mut self) -> String {
        let mut sentence = Vec::new();
    
        for _ in 0..self.num_words {
            let word = Self::generate_random_word(self);
            sentence.push(word);
        }
    
        sentence.join("-")
    }

}
