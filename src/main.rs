mod logic;

use std::io;
use std::io::BufRead;

fn main() {
    main_loop();
}

fn main_loop() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let str = line.unwrap_or_default();
        let result = match str.parse::<i32>() {
            Ok(num) => Ok(logic::num_to_string(num)),
            Err(_) => Err(""),
        };

        match result {
            Ok(result) => println!("{}", result),
            Err(_) => println!("This is not a number!"),
        }
    }
}
