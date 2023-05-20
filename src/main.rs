use std::{io::{self}, process::exit};
use calculator;

fn main() {
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Unable to read stdin");
        if user_input.trim().eq_ignore_ascii_case("quit") || user_input.trim().eq_ignore_ascii_case("q"){
            break;
        }
        //intentionally pass ownership of user_input since we don't want to use the raw input
        if let Some(result) = calculator::to_result(user_input) {
            println!("{:?}", result);
        }
    }
    exit(0);
}