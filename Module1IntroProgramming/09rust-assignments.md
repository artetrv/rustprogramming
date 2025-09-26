# Rust Programming Assignments

## Assignment 1: Temperature Converter

Create a Rust program that converts temperatures between Fahrenheit and Celsius. The program should:

1. Declare a constant for the freezing point of water in Fahrenheit (32°F).
2. Implement two functions:
   - `fahrenheit_to_celsius(f: f64) -> f64`: Converts Fahrenheit to Celsius
   - `celsius_to_fahrenheit(c: f64) -> f64`: Converts Celsius to Fahrenheit
3. In the main function:
   - Declare a mutable variable with a temperature in Fahrenheit
   - Convert it to Celsius using your function and print the result
   - Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut fahrenheit_temp: f64 = FREEZING_POINT_F;

    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.1}°F = {:.1}°C", fahrenheit_temp, celsius_temp);

    for _ in 0..5 {
        fahrenheit_temp += 1.0;
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.1}°F = {:.1}°C", fahrenheit_temp, celsius_temp);
    }
}

## Assignment 2: Number Analyzer

Create a Rust program that analyzes a series of numbers. The program should:

1. Create an array of 10 integer numbers of your choice.
2. Implement a function `is_even(n: i32) -> bool` that returns true if a number is even, false otherwise.
3. Use a for loop to iterate through the array and for each number:
   - Print whether it's even or odd using your `is_even` function
   - If the number is divisible by 3, print "Fizz" instead
   - If the number is divisible by 5, print "Buzz" instead
   - If it's divisible by both 3 and 5, print "FizzBuzz"
4. Use a while loop to find and print the sum of all numbers in the array.
5. Use a loop to find and print the largest number in the array.

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 5, 10, 12, 15, 18, 20, 25, 30, 33];

    for &num in numbers.iter() {
        if is_even(num) {
            print!("{} is Even", num);
        } else {
            print!("{} is Odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!(" -> FizzBuzz");
        } else if num % 3 == 0 {
            println!(" -> Fizz");
        } else if num % 5 == 0 {
            println!(" -> Buzz");
        } else {
            println!();
        }
    }

    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("\nSum of all numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number in the array: {}", largest);
}

## Assignment 3: Guessing Game

Create a simple number guessing game in Rust. The program should:

1. Use a mutable variable to store a "secret" number (you can hard-code this).
2. Implement a function `check_guess(guess: i32, secret: i32) -> i32` that returns:
   - 0 if the guess is correct
   - 1 if the guess is too high
   - -1 if the guess is too low
3. In the main function:
   - Use a loop to repeatedly:
     - Set a mutable guess variable to a number of your choice (simulating user input)
     - Call the `check_guess` function
     - Use an if-else expression to print whether the guess was correct, too high, or too low
     - If the guess was correct, break the loop
   - After the loop ends, print how many guesses it took (you'll need to track this in a variable)

These assignments strictly use only the concepts covered in the provided materials:
variables, mutability, basic data types (integers, booleans), arrays, functions, if-else expressions, and the three types of loops (loop, while, for). 
use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    
    let secret: i32 = 27;

    let mut attempts = 0;

    println!("Welcome to the Guessing Game!");
    println!("Try to guess the secret number.");

    loop {
       
        println!("Enter your guess: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

    
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The number was {}.", secret);
            println!("You found it in {} attempts.", attempts);
            break;
        } else if result == 1 {
            println!("Too high! Try again.");
        } else {
            println!("Too low! Try again.");
        }
    }
}



