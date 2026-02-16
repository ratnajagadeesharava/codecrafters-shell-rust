#![allow(warnings)]
use std::{
    collections::{HashMap, HashSet},
    os::unix::fs::PermissionsExt,
    path::{self, Path},
};

use std::io::{self, Write};
use std::{env, fs};

fn fill_os_commands(os_commands: &mut HashMap<String, String>) {
    for entry in fs::read_dir("/usr/bin").unwrap() {
        let path_str = entry.unwrap().path().to_string_lossy().to_string();
        let entries: Vec<&str> = path_str.split("/usr/bin/").collect();
        os_commands.insert(entries[1].to_string(), path_str);
    }
}
fn fill_path_commands(os_commands: &mut HashMap<String, String>) {
    match env::var_os("PATH") {
        Some(val) => {
            let path = val.to_string_lossy().to_string();
            let items: Vec<&str> = path.split(':').collect();
            for item in items {
                if let Ok(val) = fs::metadata(item) {
                    if val.is_dir() {
                        for entry in fs::read_dir(item).expect("not  directory") {
                            let path_str = entry.unwrap().path().to_string_lossy().to_string();
                            if let Ok(meta_data) = fs::metadata(&path_str) {
                                if meta_data.is_file() {
                                    let perm = meta_data.permissions();
                                    if perm.mode() & 0o111 != 0 {
                                        let fileName = Path::new(&path_str)
                                            .file_name()
                                            .unwrap()
                                            .to_string_lossy()
                                            .to_string();
                                        os_commands.insert(fileName, path_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            println!("no path found");
        }
    }
}

fn type_cmd(
    cmd: &Vec<&str>,
    commands: &HashSet<String>,
    os_commands: &HashMap<String, String>,
) -> String {
    if commands.contains(cmd[1]) {
        format!("{} is a shell builtin", cmd[1])
    } else if os_commands.contains_key(cmd[1]) {
        format!("{} is {}", cmd[1], os_commands.get(cmd[1]).unwrap())
    } else {
        format!("{}: not found", cmd[1])
    }
}
fn main() {
    let mut commands = HashSet::<String>::new();
    let mut os_commands: HashMap<String, String> = HashMap::<String, String>::new();
    commands.insert("exit".to_string());
    commands.insert("type".to_string());
    commands.insert("echo".to_string());
    fill_path_commands(&mut os_commands);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut commandLine: String = Default::default();
        io::stdin().read_line(&mut commandLine).unwrap();
        let cmdLine = commandLine.trim().to_string();
        let cmd: Vec<&str> = cmdLine.split(' ').collect();
        let argsLength = cmd.len();
        match cmd[0] {
            "exit" => {
                break;
            }
            "echo" => {
                println!("{}", &cmd[1..cmd.len()].join(" "))
            }
            "type" => {
                let val = type_cmd(&cmd, &commands, &os_commands);
                println!("{val}");
            }
            val  => {
                if os_commands.contains_key(val){
                    let err = exec::Command::new(val).args(&cmd[1..argsLength]).exec();
                    println!("error:{:?}",err);

                }
                else{
                    println!("{}: command not found", cmd[0]);
                }
            }
        }
    }
}
