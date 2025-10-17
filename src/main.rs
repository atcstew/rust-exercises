use std::io;

fn exercise1() {
    println!("What is your name? ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to get input");
    println!("Hello, {}, nice to meet you!", name.trim());
}

fn exercise2() {
    println!("What is the input string? ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to get input");
    let count = word.trim().len();
    println!("{} has {count} characters.", word.trim());
}

fn exercise3() {
    println!("What is the quote? ");
    let mut quote = String::new();
    let mut author = String::new();

    io::stdin().read_line(&mut quote).expect("Failed to get input");
    quote = quote.trim().to_string();
    println!("Who said it? ");
    io::stdin().read_line(&mut author).expect("Failed to get input");
    author = author.trim().to_string();
    println!("{author} says \"{quote}\".");
}

fn exercise4() {

}

fn main() {
    //exercise1();
    //exercise2();
    //exercise3();
    exercise4();
}
