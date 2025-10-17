use std::io::{self, Write};

fn main() {
    println!("\n=================\n⏱️  RUSTY COUNTER!\n=================\n");
    rusty_counter();
}

fn rusty_counter() {
    println!("Hint: Enter pure numbers e.g. 1\n");

    println!("Here are available options:");
    println!("1. Add 1 to counter");
    println!("2. Remove 1 from counter");
    println!("3. Quit the counter");

    // Store the counts
    let mut counter: u32 = 0;

    loop {
        println!("\n===== COUNTER: {} =====", counter);

        print!("\nPlease enter your option: ");

        // keep message and input on one line
        io::stdout().flush().unwrap();

        // Prompt the user to choose an option
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        // Clean user input to return u32
        let clean_input: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("> Invalid option! Please enter 1, 2, or 3.");
                continue;
            } // skip to next loop iteration
        };

        // Match option to correct action
        match clean_input {
            1 => increment(&mut counter),
            2 => decrement(&mut counter),
            3 => {
                println!("> Counter quitted!");
                println!("> Total counted: {}", counter);
                break;
            } // break loop. show total. quit counter.
            _ => println!("> Unknown option"),
        }
    }
}

fn increment(counter: &mut u32) {
    *counter += 1;

    println!("> Added 1 to counter");
}
fn decrement(counter: &mut u32) {
    if *counter != 0 {
        *counter -= 1;

        println!("> Removed 1 from counter");
    } else {
        println!("> No counts yet!");
    }
}
