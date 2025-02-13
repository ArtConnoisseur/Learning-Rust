use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!\n\n");
    // Using the rand crate to get random numbers between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // The loop keyword runs a block of code in an infinite loop

    loop {
        println!("Please input your guess.");

        // new() here is a function that is used to create an instance ofn
        // string and return it and is usually the function to look for to create
        // instances of various types? (classes)

        // TODO: Look up how classes work in rust

        let mut guess = String::new();

        // Guess is now a 32-bit integer. *The trim method is needed because a
        // newline character is included in this string that needs removal
        // REMEMBER: References are immutable by nature, so we need to pass it by &mut guess

        io::stdin() // Returns and instance of Stdin
            .read_line(&mut guess)// The & represents a reference like it does in C and C++
            .expect("Failed to read line");

        // Note that you need to change the string guess to an integer,
        // which can be done this way:


        // The match prevents the game from crashing when you enter a non integer
        // value
        let guess : u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed, {guess}, which is:");


        // Compare the generated number with the guess from the user:
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}