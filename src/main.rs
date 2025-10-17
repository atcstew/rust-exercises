use std::io;

fn exercise1() {
    println!("What is your name? ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to get input");
    println!("Hello, {}, nice to meet you!", name.trim());
}

fn main() {
    exercise1();
}
