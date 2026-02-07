use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut commands = HashSet::<String>::new();
    commands.insert("exit".to_string());
    commands.insert("type".to_string());
    commands.insert("echo".to_string());
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
            "type"=>{
                if commands.contains(cmd[1]){
                    println!("{} is a shell builtin",cmd[1]);
                }
                else{
                    println!("{}: not found",cmd[1]);
                }
            }
            _=>{
                println!("{}: command not found",cmd[0]);
            }
        }
    }
}
