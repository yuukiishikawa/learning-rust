use std::io::{self,Write};

fn main() {
    println!("Hello, world!");
    println!("aa: {0}", "Hello");

    let _ = write!(&mut io::stdout(), "Undernearth, it's all writing to a stream ...");
    println!();

    let mut input = String::new();
    if let Ok(n) = io::stdin().read_line(&mut input) {
        println!("You write: {} ({} bytes)", input,n);
    } else {
        println!("There was a error:(");
    }
}
