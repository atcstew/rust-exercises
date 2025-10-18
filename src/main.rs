use std::io;

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
        5..=60 => todo!(),
        _ => panic!("Exercise doesn't exist"),
    };
}
