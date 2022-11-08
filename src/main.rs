use rand::Rng;
use regex::Regex;
use std::io::{self, Write};
use std::fs;

fn main() {
    let wordlist_exists = std::path::Path::new("wordlist.txt").exists();

    if !wordlist_exists {
        println!("Create a list of words and slap it into wordlist.txt.");
        return
    }
    
    let regx = Regex::new(r"[A-Za-z]").unwrap();
    let mut rng = rand::thread_rng();
    let contents = fs::read_to_string("wordlist.txt").expect("Unable to read file");
    let words: Vec<&str> = contents.split("\n").collect();
    let length = words.len();
    let random_number = rng.gen_range(0..length);
    let word = words[random_number];
    let word_obfuscate = regx.replace_all(&word, "_");
    let mut word_obfuscate_chars: Vec<char> = word_obfuscate.chars().collect();
    let mut tries = 0;

    println!("Hangman! Type ! to guess the word.");
    println!("Word has {} letters.", word.len());
    loop {
        print!(">> ");

        let mut guess = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut guess).expect("Invalid input");

        let char_vector: Vec<char> = guess.trim().chars().collect();
        let mut c = ' ';
        if char_vector.len() > 0 {
            c = char_vector[0];
        }

        if c == '!' {
            let mut answer = String::new();
            print!("Guess the word: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut answer).expect("Invalid input");

            if answer.trim() == word {
                println!("Nice! You got it in {tries} tries.");
                break;
            } else {
                println!("Incorrect");
                tries += 1;
            }
        }

        let indexes = get_indexes(&word, c);

        for i in indexes {
            word_obfuscate_chars[i] = c;
        }

        let word_join: String = word_obfuscate_chars.iter().collect();
        println!("> {word_join}");
        tries += 1;

        if word_join == word {
            println!("You win! Took {tries} tries.");
            break;
        }
    }
}

fn get_indexes(word: &str, character: char) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    let word_split: Vec<char> = word.chars().collect();
    let length = word_split.len();
    let mut i = 0;

    while i < length {
        if word_split[i] == character {
            indexes.push(i);
        }
        i += 1;
    }

    indexes
}