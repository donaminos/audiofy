use std::env::args;
use url::Url;

fn is_valid_url(url: &str) -> bool {
    let result = Url::parse(url);
    result.is_ok() // Without ; at end, the value is returned automatically. It's a shorthand for return result.is_ok();
}

fn main() {
    let args: Vec<String> = args().collect();
    println!("Audiofy: Transform your favorites articles to a podcast 🚀");
    
		for (index, arg) in args.iter().skip(1).enumerate() {
        if is_valid_url(arg) {
            println!("- Valid URL at index {}: {}", index, arg);
        } else {
            println!("- Invalid argument at index {}: {}", index, arg);
        }
    }
}