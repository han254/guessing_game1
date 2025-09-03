// Guessing Game - Capstone Project with Interactive Loop
// A fully interactive number guessing game implementation in Rust

use std::io;

fn main() {
    // === GAME INTRODUCTION ===
    println!("🎯 Welcome to the Number Guessing Game!");
    println!("═══════════════════════════════════════");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess what it is?");
    println!("💡 Tip: Keep guessing until you find it!");
    println!();

    // === GAME SETUP ===
    // TODO: Generate random number (we'll add this next)
    let secret_number = 42; // Temporary hardcoded number for testing
    let mut guess_count = 0; // Track number of guesses
    
    println!("🔍 Debug: The secret number is {}", secret_number);
    println!("(This debug line will be removed later)");
    println!();

    // === MAIN GAME LOOP ===
    // This loop continues until the user guesses correctly
    loop {
        guess_count += 1;
        println!("📝 Guess #{}: Please input your guess:", guess_count);

        // === USER INPUT HANDLING ===
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("❌ Failed to read your input");

        // === INPUT PROCESSING WITH IMPROVED ERROR HANDLING ===
        // Remove whitespace and convert to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                // Input validation: Check if number is in valid range
                if num < 1 || num > 100 {
                    println!("⚠️  Please enter a number between 1 and 100!");
                    println!("   You entered: {}", num);
                    continue; // Skip to next iteration, ask for input again
                }
                println!("✅ You guessed: {}", num);
                num
            },
            Err(_) => {
                println!("❌ '{}' is not a valid number!", guess.trim());
                println!("   Please enter a whole number between 1 and 100.");
                continue; // Skip to next iteration, ask for input again
            }
        };

        // === GAME LOGIC WITH COMPARISON ===
        if guess == secret_number {
            // === WIN CONDITION - EXIT THE LOOP ===
            println!();
            println!("🎉 Congratulations! You guessed the number!");
            println!("🏆 You won in {} guess{}!", 
                     guess_count, 
                     if guess_count == 1 { "" } else { "es" });
            println!("🌟 The secret number was indeed {}!", secret_number);
            break; // Exit the loop - game over!
        } else if guess < secret_number {
            println!("📈 Too small! Try a bigger number.");
        } else {
            println!("📉 Too big! Try a smaller number.");
        }

        // === ENCOURAGEMENT FOR NEXT GUESS ===
        println!("   Keep trying! 💪");
        println!();
    }

    // === GAME CONCLUSION ===
    println!();
    println!("Thanks for playing! 🎮");
    
    // === PERFORMANCE FEEDBACK ===
    match guess_count {
        1 => println!("🤯 Incredible! You got it in one guess!"),
        2..=5 => println!("🎯 Excellent guessing! Very efficient!"),
        6..=10 => println!("👍 Good job! Nice strategy!"),
        _ => println!("🎲 Persistence pays off! Well done!"),
    }
}