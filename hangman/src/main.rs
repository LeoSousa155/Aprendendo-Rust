use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut word = String::new();
    println!("Write the secret word:");
    io::stdin().read_line(&mut word)
        .expect("Failed to read word!");


    let mut errors = 0;

    loop {
        print_hangman(errors);

        let letter: String = input_letter();
        println!("The letter '{}' was inserted.", letter);

        if errors >= 7 {
            println!("YOU LOSE");
            break;
        } 
        errors = errors + 1
    }
}


/// will print the hangman based on the total errors of the player
fn print_hangman(state: usize) {
    //hangman is an array with all hangman ASCII draws
    let hangman = [
    "
    ------------
    |        
    |        
    |        
    |       
    |
    -------------------",
    "
    ------------
     |        |
     |        
     |        
     |       
     |
    -------------------",
    "
    ------------
     |        |
     |        O
     |        
     |       
     |
    -------------------",
    "
    ------------
     |        |
     |        O
     |        |
     |       
     |
    -------------------",
    "
    ------------
     |        |
     |        O
     |       /|
     |       
     |
    -------------------",
    "
    ------------
     |        |
     |        O
     |       /|\\
     |       
     |
    -------------------",
    "
     ------------
     |        |
     |        O
     |       /|\\
     |       /
     |
    -------------------",
    "
     ------------
     |        |
     |        O
     |       /|\\
     |       / \\
     |
    -------------------"
    ];
    if state >= hangman.len() {
        return
    }
    println!("{}", hangman[state]);
}


/// will handle the under character input to prevent him to introduce multiple or null characters
fn input_letter() -> String {
    let mut input_letter = String::new();

    loop {
        input_letter = String::from("");
        println!("Guess a letter:");
        io::stdin().read_line(&mut input_letter)
            .expect("Failed to read word!");
        //will clear \r\n characters after the input
        input_letter.pop();
        input_letter.pop();

        if input_letter.graphemes(true).count() > 1 || input_letter.graphemes(true).count() == 0 {
            println!("Insert only one letter per guess.");
            continue;
        }
        break;
    }
    return input_letter
}

fn print_word() {
    println!("Nothing")
}

