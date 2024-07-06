use std::char;
use rand::Rng;

pub struct PassGen {
    size: usize,
    uppercase: String,
    numbers: String,
    symbols: String,
}

impl PassGen {

    pub fn new(
        size: usize,
        uppercase: String,
        numbers: String,
        symbols: String, 
    ) -> Self {
        PassGen {
            size,
            uppercase,
            numbers,
            symbols,
        }
    }

    fn generate_random_password(&mut self) -> String {
        let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

        if self.uppercase == "y" {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }

        if self.numbers == "y" {
            charset.push_str("0123456789");
        }

        if self.symbols == "y" {
            charset.push_str("!@#$%^&*()_-+=<>?");
        }

        let charset = charset.as_bytes();
        let mut rng = rand::thread_rng();
        let password: String = (0..self.size)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        password
    }

    pub fn get(&mut self) -> String {
        let password = self.generate_random_password();

        println!("---------------------------------");
        println!("Your password: {}", password);

        password
    }

}
