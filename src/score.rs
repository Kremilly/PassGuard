use zxcvbn::zxcvbn;

pub struct PassScore {
    password: String,
    show_score: String,
}

impl PassScore {

    fn strength(&self, score: u8) -> String {
        match score {
            0 => "Very weak".to_string(),
            1 => "Weak".to_string(),
            2 => "Fair".to_string(),
            3 => "Strong".to_string(),
            4 => "Very strong".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn new(password: String, show_score: String) -> Self {
        PassScore {
            password: password.to_string(),
            show_score: show_score.to_string(),
        }
    }

    pub fn get(&mut self) {
        if self.show_score.to_lowercase() != "y" {
            return;
        }
        
        let result = zxcvbn(&self.password, &[]);

        let score = result.score() as u8;
        let strength = self.strength(score);
        let crack_time = result.crack_times().online_throttling_100_per_hour();

        println!("---------------------------------");
        println!("Password strength: {} (Score: {}/4)", strength, score);
        println!("Crack time (Online with throttling (100 per hour)): {}", crack_time);
    }

}
