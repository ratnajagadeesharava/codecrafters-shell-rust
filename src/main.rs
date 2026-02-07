#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut command:String =Default::default();
    io::stdin().read_line(&mut command).unwrap();
    let cmd = command.trim().to_string();
    println!("{cmd}: command not found");
}
