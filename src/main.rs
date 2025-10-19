use chrono::Datelike;
use num::*;
use std::io;

fn get_current_year() -> u32 {
    let year = chrono::Utc::now().year();
    clamp(0, year.try_into().unwrap(), u32::MAX)
}

fn prompt(question: &str) -> String {
    println!("{question}: ");
    let mut var = String::new();
    io::stdin().read_line(&mut var).expect("Failed to get input");
    var.trim().to_string()
}

fn exercise1() {
    println!("Hello, {}, nice to meet you!", prompt("What is your name?"));
}

fn exercise2() {
    let word = prompt("What is the input string?");
    println!("{} has {} characters.", word, word.len());
}

fn exercise3() {
    let quote = prompt("What is the quote?");
    let author = prompt("Who said it?");
    println!("{author} says \"{quote}\".");
}

fn exercise4() {
    let noun = prompt("Enter a noun");
    let verb = prompt("Enter a verb");
    let adjective = prompt("Enter an adjective");
    let adverb = prompt("Enter an adverb");

    println!("Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!");
}

fn exercise5() {
    let first = prompt("Enter the first number")
        .parse::<i32>()
        .expect("Please enter an integer");
    let second = prompt("Enter the second number")
        .parse::<i32>()
        .expect("Please enter an integer");
    let sum = first + second;
    let difference = first - second;
    let product = first * second;
    let quotient = first / second;

    println!("{first} + {second} = {sum}");
    println!("{first} - {second} = {difference}");
    println!("{first} * {second} = {product}");
    println!("{first} / {second} = {quotient}");

}

fn exercise6() {
    let current_age = prompt("How old are you?")
        .parse::<u32>()
        .expect("Please enter an integer");
    let retirement_age = prompt("At what age do you want to retire?")
        .parse::<u32>()
        .expect("Please enter an integer");
    if retirement_age <= current_age {
        println!("You can already retire!")
    } else {
        let working_years = retirement_age - current_age;
        let current_year = get_current_year();
        let retirement_year = current_year + working_years;
        println!("You have {working_years} year(s) left until you can retire.");
        println!("It is {current_year}, so you can retire in {retirement_year}.");
    }
}

fn main() {
    let exercise = prompt("Which exercise do you want to run?")
        .parse::<u8>()
        .expect("Please enter an integer");
    match exercise {
        0 => (),
        1 => exercise1(),
        2 => exercise2(),
        3 => exercise3(),
        4 => exercise4(),
        5 => exercise5(),
        6 => exercise6(),
        7..=57 => todo!(),
        _ => panic!("Exercise doesn't exist"),
    };
}
