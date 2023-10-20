use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process::exit;
use std::env;
use std::fs;
//use tokio; last high level crate for a CURL alternative and async
//use std::ptr; bare metal graphics version soon


fn main() {
    const VERSION: i8 = 2; 
    loop {
        print!("# > ");
        stdout().flush().unwrap(); 

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next(); 
        let args: Vec<&str> = parts.collect(); 

        match command {
            Some("help") => help(),
            Some("-h") => help(),
            Some("?") => help(),
            Some("exit") => {
                println!("Exiting...");
                exit(1)
            },
            Some("cat") => {
                if let Some(file_name) = args.first() {
                    cat(file_name);
                } else {
                    println!("cat command needs a filename");
                }
            },
            Some("write") => {
                if let Some(file_name) = args.first() {
                    write_file(file_name);
                } else {
                    println!("write command needs a filename");
                }
            },
            Some("ls") => ls(),
            Some("pwd") => {
                let current_dir = pwd(); 
                println!("{}", current_dir); 
            },
            Some("clear") => {
                print!("\x1B[2J\x1B[1;1H");
                stdout().flush().unwrap();
            },
            Some("version") => println!("RUSTerminal version : {} High Level version date 20/10/2023", VERSION),
            Some(cmd) => println!("Unknown command: '{}'. Type 'help' for available commands.", cmd),
            None => {} 
        }
    }
}

fn help(){
    println!("help or -h or ? - Shows this message");
    println!("ls - Lists the files in the current directory");
    println!("pwd - Shows the current directory");
    println!("cat - Reads a file");
    println!("write - Writes to a file");
    println!("version or -v - Shows the version");
    println!("clear - Clears the screen");
    println!("exit - Exits the program");
}

fn pwd() -> String{
    let path = env::current_dir().unwrap();
    return path.to_str().unwrap().to_string();
}

fn ls() {
    let path = env::current_dir().unwrap();
    let items = std::fs::read_dir(path).unwrap();

    for item in items {
        let entry = item.unwrap();
        let path = entry.path();

        if let Some(name) = path.file_name(){
            if path.is_dir(){
                println!("{}/", name.to_str().unwrap());
            } else {
                println!("{}", name.to_str().unwrap());
            }
        }
    }
}

fn cat(file_name: &str) {
    let full = pwd();
    let full_path = format!("{}/{}", full, file_name);
    match fs::read_to_string(full_path) {
        Ok(contents) => println!("{}", contents),
        Err(err) => println!("Error reading file: {}", err),
    }
}

fn write_file(file_name: &str) {
    let full = pwd();
    let full_path = format!("{}/{}", full, file_name);
    let mut contents = String::new();
    println!("Enter file content:");
    stdin().read_line(&mut contents).unwrap();
    match fs::write(full_path, contents) {
        Ok(_) => println!("File written successfully"),
        Err(err) => println!("Error writing file: {}", err),
    }
}