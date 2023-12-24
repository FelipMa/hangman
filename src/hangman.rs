use ferris_says;
use std::io::{stdout, BufWriter};

pub struct Game {
    word: String,
    max_misses: u8,
    missed_letters: Vec<char>,
    hits: Vec<bool>,
    correct_letters: Vec<char>,
    attempts: u8,
}

impl Game {
    pub fn new(word: String) -> Game {
        Game {
            word: word.trim().to_string(),
            max_misses: 6,
            missed_letters: Vec::new(),
            hits: vec![false; word.len()],
            correct_letters: Vec::new(),
            attempts: 0,
        }
    }

    pub fn check_finish(&self) -> bool {
        if self.missed_letters.len() as u8 > self.max_misses {
            let message = format!("You lost! The word was \"{}\".", self.word);
            let width = message.chars().count();
            let mut writer = BufWriter::new(stdout().lock());
            ferris_says::say(&message, width, &mut writer).unwrap();
            return true;
        }

        for i in 0..self.word.len() {
            if !self.hits[i] {
                return false;
            }
        }

        let message = format!("Congratulations! You won! The word was \"{}\".", self.word);
        let width = message.chars().count();
        let mut writer = BufWriter::new(stdout().lock());
        ferris_says::say(&message, width, &mut writer).unwrap();
        return true;
    }

    pub fn print_state(&self) {
        println!("Attempt: {}", self.attempts);
        println!("\nWord:\n");
        for i in 0..self.word.len() {
            if self.hits[i] {
                print!("{} ", self.word.chars().nth(i).unwrap());
            } else {
                print!("_ ");
            }
        }
        println!("\n");

        print!("Missed letters: {:?}", self.missed_letters);
        println!(" ({}/{})", self.missed_letters.len(), self.max_misses);

        println!("Correct letters: {:?}", self.correct_letters);
    }

    pub fn guess(&mut self, letter: char) {
        for i in 0..self.missed_letters.len() {
            if self.missed_letters[i] == letter {
                println!("You already tried this letter! Try again.");
                return;
            }
        }

        for i in 0..self.correct_letters.len() {
            if self.correct_letters[i] == letter {
                println!("You already tried this letter! Try again.");
                return;
            }
        }

        self.attempts += 1;

        let mut found = false;
        for i in 0..self.word.len() {
            if self.word.chars().nth(i).unwrap() == letter {
                self.hits[i] = true;
                found = true;
            }
        }
        
        println!(".......................................................");
        if found {
            println!("You got it!");
            self.correct_letters.push(letter);
        } else {
            println!("Wrong letter!");
            self.missed_letters.push(letter);
        }

        self.print_state();
        println!(".......................................................");
    }
}