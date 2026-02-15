#![allow(warnings)]
use std::{
    collections::{HashMap, HashSet}, os::unix::fs::PermissionsExt, path::{self, Path}
};

use std::io::{self, Write};
use std::{env, fs};
fn main() {
    let mut commands = HashSet::<String>::new();
    let mut os_commands: HashMap<String, String> = HashMap::<String, String>::new();
    commands.insert("exit".to_string());
    commands.insert("type".to_string());
    commands.insert("echo".to_string());
    match env::var_os("PATH") {
        Some(val) => {
            let path = val.to_string_lossy().to_string();
            let items: Vec<&str> = path.split(':').collect();

            for item in items {
                
                if let Ok(val) = fs::metadata(item) {
                    if val.is_dir() {
                        for entry in fs::read_dir(item).expect("not  directory") {
                            let path_str = entry.unwrap().path().to_string_lossy().to_string();
                            let meta_data = fs::metadata(&path_str).unwrap();

                            if meta_data.is_file() {
                                let p = Path::new(&path_str);
                                
                                let perm = meta_data.permissions();
                                
                            }
                        }
                    }
                }

                // println!("{:?}",meta.is_dir());
            }
        }
        None => {
            println!("no path found");
        }
    }
    for entry in fs::read_dir("/usr/bin").unwrap() {
        let path_str = entry.unwrap().path().to_string_lossy().to_string();
        let entries: Vec<&str> = path_str.split("/usr/bin/").collect();
        os_commands.insert(entries[1].to_string(), path_str);
    }
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut commandLine: String = Default::default();
        io::stdin().read_line(&mut commandLine).unwrap();
        let cmdLine = commandLine.trim().to_string();
        let cmd: Vec<&str> = cmdLine.split(' ').collect();
        match cmd[0] {
            "exit" => {
                break;
            }
            "echo" => {
                println!("{}", &cmd[1..cmd.len()].join(" "))
            }
            "type" => {
                if commands.contains(cmd[1]) {
                    println!("{} is a shell builtin", cmd[1]);
                } else if os_commands.contains_key(cmd[1]) {
                    println!("{} is {}", cmd[1], os_commands.get(cmd[1]).unwrap());
                } else {
                    println!("{}: not found", cmd[1]);
                }
            }
            _ => {
                println!("{}: command not found", cmd[0]);
            }
        }
    }
}
