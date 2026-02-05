
fn ass1(){
    const FREEZING_POINT: f64 = 32.0;

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) / 1.8
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        (c * 1.8) + 32.0
    }

    let mut ftemp = FREEZING_POINT;    
    let mut ctemp;
    
    println!("01: Temperature Converter");
    println!("Freezing Point in celsius: {}°C",fahrenheit_to_celsius(ftemp));
    let mut counter = 0;
    while counter < 5 {
        ftemp += 1.0;
        ctemp = fahrenheit_to_celsius(ftemp);
        println!("{}°C = {}°F", ctemp, celsius_to_fahrenheit(ctemp));
        counter += 1;
    }
}

fn ass2(){
    println!("02: Number Analyzer");
    fn is_even(n: i32) -> bool {
        0 == n % 2
    }

    let int_array: [i32; 10] = [-7, 3, 6, 15, 26, 33, -4, 0, -11, -10];

    for i in 0..10 {
        if int_array[i] % 5 == 0 && int_array[i] % 3 == 0 {
            print!("FizzBuzz ");
        } else if int_array[i] % 5 == 0 {
            print!("Buzz ");
        } else if int_array[i] % 3 == 0 {
            print!("Fizz ");
        } else if is_even(int_array[i]) {
            print!("Even ");
        } else {
            print!("Odd ");
        }
    }
    println!();

    let mut counter = 0;
    let mut sum = 0;
    while counter < 10 {
        sum += int_array[counter];
        counter += 1;
    }
    println!("Total sum of array: {}", sum);

    let mut biggest = 0;
    for i in 0..10 {
       if i == 0 {
        biggest = int_array[i];
       }
       if biggest < int_array[i] {
        biggest = int_array[i];
       }
    }
    println!("Biggest number in array: {}", biggest);
}

fn ass3(){
    use std::io::{self, Write};

    println!("03: Guessing Game");
    io::stdout().flush().unwrap();
    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess > secret {
            1
        } else if guess < secret {
            -1
        } else {
            0
        }
    }

    let mut secret = 4;
    let mut guess = String::new();
    let mut number: i32;
    let mut attempts = 0;

    print!("Take a guess: ");
    io::stdout().flush().unwrap();
    
    loop {
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        number = guess.trim().parse().expect("Please enter a valid number.");
        
        attempts += 1;
        if check_guess(number, secret) == 0 {
            println!("Congratulations you guessed right!");
            println!("It took you this many guesses: {}", attempts);
            break;
        } else if check_guess(number, secret) == 1 {
            println!("Sorry, your guess is too high.");
        } else {
            println!("Sorry, your guess is too low.");
        }

        print!("Guess again: ");
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let name = "Heriberto Gutierrez Jr.";
    let course = "CSCI-3334-02";

    println!("Module 01: 09 Assignments");
    println!("{}",name);
    println!("{}",course);

    println!();
    ass1();
    println!();
    ass2();
    println!();
    ass3();
}