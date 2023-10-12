#![deny(clippy::all)]
// use the standard rust library.
use std::*;

fn main() {
    // Declare a variable
    let number = 42;

    // Simple if statement
    if number > 0 {
        println!("{} is a positive number", number);
    }

    // If-else statement
    if number % 2 == 0 {
        println!("{} is an even number", number);
    } else {
        println!("{} is an odd number", number);
    }

    // Also can be written using a match statement
    match number % 2 {
        0 => println!("{} is an even number", number),
        _ => println!("{} is an odd number", number), // The underscore is the else statement.
    }

    // When creating larger if else and else if statements using match makes it very simple.
    // Using cmp we can create a match compare statement.
    // The .cmp(&0) tells the match statement to compare number with 0.
    // Then select the best outcome from the options provided in the curly brackets.
    match number.cmp(&0) {
        cmp::Ordering::Less => println!("{} is a negative number", number),
        cmp::Ordering::Equal => println!("{} is zero", number),
        cmp::Ordering::Greater => println!("{} is a positive number", number),
    }

    // Using if in a let statement. This is cool and new to me.
    // This is basically a ternary operator just extended with the if statement.
    let result = if number > 0 {
        "positive"
    } else {
        "non-positive"
    };

    println!("The number is {}", result);

    // I was currios if you could use match statements in the same way
    // Sure enough you can.
    let test_result = match number.cmp(&0) {
        cmp::Ordering::Less => "a negative number",
        cmp::Ordering::Equal => "zero",
        cmp::Ordering::Greater => "a positive number",
    };

    println!("Test result is {}", test_result);
}
