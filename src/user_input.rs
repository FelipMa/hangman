use std::io;
use std::io::Write;

pub fn get_input(prompt: &str) -> char {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let treated_input = input.trim().to_string();

    if treated_input == "quit" {
        std::process::exit(0);
    }

    if treated_input.len() != 1 {
        if treated_input.len() == 0 {
            println!("You must type something! Try again.");
        }
        if treated_input.len() > 1 {
            println!("You must type only one character! Try again.");
        }
        return get_input(prompt);
    }

    let input_char = treated_input.chars().nth(0).unwrap();

    if !input_char.is_alphabetic() {
        println!("You must type a letter! Try again.");
        return get_input(prompt);
    }

    return input_char
}
