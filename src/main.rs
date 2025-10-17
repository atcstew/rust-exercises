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

fn main() {
    //exercise1();
    exercise2();
}
