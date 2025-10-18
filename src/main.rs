use std::io;

fn prompt(question: &str) -> String {
    println!("{question}: ");
    let mut var = String::new();
    io::stdin().read_line(&mut var).expect("Failed to get input");
    var.trim().to_string()
}

fn exercise1() {
    let name = prompt("What is your name?");
    println!("Hello, {name}, nice to meet you!");
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

fn main() {
    //exercise1();
    //exercise2();
    //exercise3();
    exercise4();
}
