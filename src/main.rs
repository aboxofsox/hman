use rand::Rng;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::ops::Range;

const MAX_TRIES: usize = 20;
const VALID_LOWER: Range<u8> = 97..122;
const VALID_UPPER: Range<u8> = 65..90;

fn main() {
    // Check if the wordlist.txt exists.
    // If it doesn't, tell the user and end the program.
    let wordlist_exists = std::path::Path::new("wordlist.txt").exists();

    if !wordlist_exists {
        println!("Create a list of words and slap it into wordlist.txt.");
        return;
    }

    // Regular expression to replace letters with underscores.
    let regx = Regex::new(r"[A-Za-z]").unwrap();

    let mut rng = rand::thread_rng();

    // Load the contents of wordlist.txt.
    // Split the contents by line break (\n).
    let contents = fs::read_to_string("wordlist.txt").expect("Unable to read file");
    let words: Vec<&str> = contents.split("\n").collect();
    let length = words.len();

    // Initialize total tries.
    let mut tries = 0;

    // Generate a random number between 0 and the length of words in wordlist.txt.
    let random_number = rng.gen_range(0..length);

    // Get a random word from the wordlist.
    let word = words[random_number];

    // Replace all letters with underscroes.
    // Then convert it to a char vector.
    let word_obfuscate = regx.replace_all(&word, "_");
    let mut word_obfuscate_chars: Vec<char> = word_obfuscate.chars().collect();

    // Create a vector for all the letters guessed.
    let mut guessed_letters: Vec<char> = Vec::with_capacity(MAX_TRIES);

    println!("Hangman! Type ! to guess the word.");
    println!("Word has {} letters.", word.len());

    // Start the game loop.
    loop {
        print!(">> ");

        // Create string buffer for user input.
        let mut guess = String::new();
        // Readline from the same line as the previous println.
        let _ = io::stdout().flush();
        // Get the user input.
        io::stdin().read_line(&mut guess).expect("Invalid input");

        // Convert the input into char vector.
        // Because it's input from the terminal, it needs to be trimmed.
        let char_vector: Vec<char> = guess.trim().chars().collect();
        // The default character should just be a blank character.
        let mut c = ' ';
        // Only a single character is needed, so it will always be the first character typed, regardless of input length.
        // An empty input is invalid and has a length of 0.
        if char_vector.len() > 0 {
            c = char_vector[0];
        }

        // If we meed or exceed the number of max tries, break the game loop.
        if guessed_letters.len() >= MAX_TRIES {
            println!("Game Over.");
            break;
        }

        // If the letter hasn't bee guessed, push it to the char vector.
        if !guessed_letters.contains(&c) && is_valid(c) {
            guessed_letters.push(c);
        }

        // Make the vector a little less ugly to print.
        let guessed_letters_string: String = guessed_letters.iter().collect();
        println!("Guessed: {}\n", guessed_letters_string);

        // If the word is known, a ! can be used to input the entire word.
        if c == '!' {
            let mut answer = String::new();
            print!("Guess the word: ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut answer).expect("Invalid input");

            if answer.trim() == word {
                println!("Nice! You got it in {tries} tries.");
                break;
            }
            println!("Incorrect");
            tries += 1;
        }

        // Get the indexes of each occurence of a letter.
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

// Pass in a word and a character.
// Return a vector containing the character's index. Including repeated letters.
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

// Return false if the character is not a letter (either upper or lowercase).
fn is_valid(c: char) -> bool {
    let c_as_u8 = c as u8;
    VALID_UPPER.contains(&c_as_u8) || VALID_LOWER.contains(&c_as_u8)
}
