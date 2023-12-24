mod user_input;
mod hangman;
mod get_word;

fn main() {
    println!("\nWelcome to hangman!\n\nTo quit, type \"quit\" or press CTRL+C\n");

    fn play() {
        println!("Would you like to play in wich language?\n1 - English\n2 - Brazilian Portuguese");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let treated_input = input.trim().to_string();

        if treated_input == "quit" {
            println!("Thanks for playing!");
            std::process::exit(0);
        }

        if treated_input != "1" && treated_input != "2" {
            println!("Invalid option! Try again.");
            return play();
        }

        println!("Randomizing word...");

        let mut game = hangman::Game::new(get_word::get_word(treated_input));

        println!(".......................................................");

        while !game.check_finish() {
            let letter = user_input::get_input("Enter a letter: ");
    
            game.guess(letter);
        }
    }

    loop {
        play();

        println!("Do you want to play again? (y/n)");
        let letter = user_input::get_input("");

        if letter == 'n' {
            break;
        }

        println!(".......................................................");
    }

    println!("Thanks for playing!");
}
