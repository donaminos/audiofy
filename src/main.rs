use std::env;

fn main() {
   let msg = "Audiofy: turn your favorite articles into a podcast! 🚀";
    println!("{}", msg); 

    let args: Vec<String> = env::args().collect();

    for (index, arg) in args.iter().skip(1).enumerate() {
      println!("- Arg at index {}: {}", index, arg);
    }
}