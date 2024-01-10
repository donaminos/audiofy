use std::env;

fn main() {
    let msg = "Audiofy: turn your favorite articles into a podcast! 🚀";
    println!("{}", msg); 

    let args: Vec<String> = env::args().collect();

    // args[0] is the path to the program
    // Further elements are the passed command-line arguments
    println!("Command-line arguments: {:?}", args);
}