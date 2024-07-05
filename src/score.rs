use zxcvbn::zxcvbn;

pub struct PassScore {
    password: String,
}

impl PassScore {

    pub fn new(password: &str) -> Self {
        PassScore {
            password: password.to_string(),
        }
    }

    pub fn get(&mut self) {
        let result = zxcvbn(&self.password, &[]);
        let crack_time = result.crack_times().online_throttling_100_per_hour();

        println!("---------------------------------");
        println!("Password strength: {}", result.score());
        println!("Crack time: {}", crack_time);
    }

}
