use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play GUESS THE NUMBER");

    //the random number the player need to guess
    let secret_number = rand::thread_rng().gen_range(0..101);

    let mut total_tries = 0;

    loop {
        println!("Guess the number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        total_tries += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("You won in {} total tries", total_tries);
}
