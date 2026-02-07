#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command: String = Default::default();
        io::stdin().read_line(&mut command).unwrap();
        let cmd = command.trim().to_string();
        if cmd == "exit"{
            break ;
        }
        println!("{cmd}: command not found");
    }
}
