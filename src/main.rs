use calculator;
use std::io::{self};

fn main() {
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read stdin");
        let trimmed_input = user_input.trim();
        if trimmed_input.eq_ignore_ascii_case("quit") || trimmed_input.eq_ignore_ascii_case("q") {
            return;
        }
        match calculator::parse(trimmed_input) {
            Ok(x) => println!("{x}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
