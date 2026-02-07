#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut commandLine: String = Default::default();
        io::stdin().read_line(&mut commandLine).unwrap();
        let cmdLine = commandLine.trim().to_string();
        let cmd:Vec<&str>  = cmdLine.split(' ').collect();
        match cmd[0]{
            "exit"=> {
                break;
            },
            "echo"=>{
                println!("{}",&cmd[1..cmd.len()].join(" "))
            },
            _=>{
                println!("{}:command not found",cmd[0]);
            }
        }
    }
}
