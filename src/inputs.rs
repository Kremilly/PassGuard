use std::io::{
    self,
    Write
};

pub struct Inputs;

impl Inputs {

    pub fn read_int(text: &str) -> usize {
        write!(io::stdout(), "{} ", text).expect("Failed to write to stdout");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        input = input.trim().to_string();
        input.parse().expect("Please enter a valid number")
    }

    pub fn read_str(text: &str) -> String {
        write!(io::stdout(), "{} ", text).expect("Failed to write to stdout");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        input.trim().to_string()
    }

}