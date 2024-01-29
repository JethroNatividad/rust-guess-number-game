use rand::Rng;
use std::io;
use std::io::Write;

// Guess number game, 3 levels. level 1: 1 - 10, Level 2: 1 - 100, Level 3: 1 - 1000. show Too high, Too low, track the attempts.
// Inputs: level, guess
// Process: get level, generate random number, loop, get guess, if > random show Too high, if < random show Too Low. ask if play again.
// Outputs: Too High, Too Low, You got it in n guesses!

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

#[derive(Debug)]
enum Difficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

fn get_difficulty() -> Difficulty {
    loop {
        let input: i32 = get_input("Pick a difficulty level (1, 2, or 3): ");
        match input {
            1 => return Difficulty::Easy,
            2 => return Difficulty::Medium,
            3 => return Difficulty::Hard,
            _ => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // get difficulty, "Pick a difficulty level (1, 2, or 3): "
    // reask if not 1, 2, or 3
    let difficulty: Difficulty = get_difficulty();
    // initialize attemps = 1
    let attemps: i64 = 1;
    let mut rng = rand::thread_rng();

    // generate random_number, based on the difficulty. 1 = 1 - 10, 2 = 1 - 100, 3 = 1 - 1000.
    let random_number: i64 = match difficulty {
        Difficulty::Easy => rng.gen_range(1..=10),
        Difficulty::Medium => rng.gen_range(1..=100),
        Difficulty::Hard => rng.gen_range(1..=1000),
    };

    println!("{}", random_number)

    // get guess, "I have my number. What's your guess? "
    let mut guess: i64 = get_input("I have my number. What's your guess? ");

    // if guessed correctly
    // get plural guess/guesses
    // print "You got it in {} guess/guesses!"

    // if guess > random_number
    // increment attempts, reask "Too high. Guess again: "

    // if guess < random_number
    // increment attempts, reask "Too low. Guess again: "

    // Ask, "Play again? "
    // reask if not y or n
    // if n, print "Goodbye!"
}
