// hangman but without the drawing (reach 5 wrong = dead)

use std::io::{self, Write};

fn main() {

    let mut is_correct = false;
    let word: String = "hello".to_string();

    while is_correct == false {
    print!("Enter your first guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    
    guess = guess.trim().to_string();
    
        if guess == word {
            println!("You won! The word was {}", word);
            is_correct = true;
        } else {
            println!("Wrong! Try again!");
        }
    }

}

