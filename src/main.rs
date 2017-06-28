mod controllers;

use std::io::{stdin, Write, stdout};

pub fn main() {
    loop {
        print!("What action do you want to perform? (q to quit): ");
        stdout().flush().unwrap();
        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap();

        let answer = answer.trim_right();

        match answer {
            "index" => controllers::tickets::index(),
            "create" => controllers::tickets::create(),
            "q" => break,
            _ => println!("I didn't get that"),
        }
    }
}
