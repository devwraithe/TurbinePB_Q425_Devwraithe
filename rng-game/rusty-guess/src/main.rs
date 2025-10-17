use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate secret number with between 1 and 60
    let secret_number = rand::thread_rng().gen_range(1..=60);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim whitespaces and match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // guess var shadow

        println!("Your guess: {guess}");

        // Check guess range
        if guess >= 1 && guess <= 60 {
            // Match guess to secret num, stop game if guess matches
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        } else {
            println!("Hint: You should between 1 and 60");
        }
    }
}
