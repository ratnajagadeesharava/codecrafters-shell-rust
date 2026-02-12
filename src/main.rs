use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env,fs};
fn main() {
    let mut commands = HashSet::<String>::new();
    commands.insert("exit".to_string());
    commands.insert("type".to_string());
    commands.insert("echo".to_string());

    for entry in fs::read_dir("/usr/bin").unwrap(){
        let path_str= entry.unwrap().path().to_string_lossy().to_string();
        let entries:Vec<&str> = path_str.split("/usr/bin/").collect();
        // println!("{:?}",entries[1]);
        commands.insert(entries[1].to_string());
    }
    // println!("{:?}",commands);
    loop {
        

        print!("$ ");
        io::stdout().flush().unwrap();
        let mut commandLine: String = Default::default();
        io::stdin().read_line(&mut commandLine).unwrap();
        let cmdLine = commandLine.trim().to_string();
        println!("{:?}",cmdLine);
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
