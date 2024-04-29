use std::io::Read;

fn main() {
    hello_name();
}

fn hello_name() {
    let mut name = String::new();
    println!("What is your name?");
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.trim().to_string();
    println!("Hello, {}!", name);
}