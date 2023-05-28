// example from Chapter 2 of https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io; // importing the io from the standard librar, it could still be used as std::io::stdin without importing previously
use rand::Rng; // importing the Rng trait from the rand crate
use std::cmp::Ordering; // importing the Ordering enum from the standard library, it is used to compare values

fn main() {
    println!("Guess a number!");
    println!("Your input: ");

    let mut guess = String::new(); // mutable variable bound to a new, empty instance of a String
    let secret_number = rand::thread_rng().gen_range(1..=100); // generating a random number between 1 and 100

    /*
    Ranges in Rust 
    1..=100 is a range, it includes the last number
    1..100 is a range, it does not include the last number
    */

    /* 
    a::b means that b is a function defined in a
    a.b means that b is a method defined on the instance a
    */ 

    io::stdin()
        .read_line(&mut guess) // passing a mutable refference to the guess variable
        .expect("Failed to read line");
    
    /* 
    .read_line() return the Result type, it has two possible states - variants - Ok or Err
    Ok means that the operation was successful
    Err means that the operation failed and includes information about how or why the operation failed
    For Err we will fallback onto the expect method, which will crash the program and display the message passed as argument

    Without using .expect() it would still compile, but with a warning
    */

    /*   
    Rust defaults to i32 when the int type is not specified, therefor secret_number is an i32 and guess is a String
    To compare these two values we need to convert guess to an i32
    */

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing the guess variable, converting it to an u32 (unsigned 32-bit integer)

    // Tell no one the secret number is only kept a secret by those two slashes
    // println!("You guessed: {guess}, the secret number is: {secret_number}");

    /* 
    .trim() - removes whitespaces
    .parse() - parses a string into some kind of number
    .expect() - crashes the program if user input is not a number and displays the message passed as argument
    */

    // matching numbers
    match guess.cmp(&secret_number) { 
        Ordering::Less => println!("Too small!"), // examples of pattern matching, each of these is called an arm
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // Further attempts

    let mut guess = String::new(); // it has to be either redefined or variable.clean()

    /*  
    // infinite loop, but naive number parsing, that will result in crash if invalid number is typed
    loop {
        println! ("Try again :(");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess_int32: u32 = guess.trim().parse().expect("Please type a number!");

        match guess_int32.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Noice!");
                println!("The game ends here...");
                break;
            }
        }

        guess.clear() // guess has to be cleared, instead data would be append with \n at every input, for example: "10\n1\n26\n"
    } 
    */

    // infinite loop, with two variants for parsing the typed number, that will handle invalid number without crashing
    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // instead of .expect(), as a fallback for wrong input we can just handle both positive and failed parses
        let guess_int32: u32 = match guess.trim().parse() {
            Ok(num) => num, // variants are returned containing the result, thats way both arms have (something), if Ok we want to get the result
            Err(_) => { // in case of error we do not care about the errortype or whatever is being return so catchall _ is used
                println!("THAT WAS NOT A PROPER NUMBER");
                guess.clear();
                continue;
            },
        };

        match guess_int32.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Noice!");
                println!("The game ends here...");
                break;
            }
        }

        guess.clear() // guess has to be cleared, instead data would be append with \n at every input, for example: "10\n1\n26\n"
    }
}
