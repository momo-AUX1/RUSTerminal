use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process::exit;
use std::env;
use std::fs;
use rlua::Lua;
use std::process;

#[tokio::main]
async fn main() {
    const VERSION: i8 = 3; 
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
            Some("exec") => {
                if let Some(file_name) = args.first() {
                    file_runner_handler(file_name);
                } else {
                    println!("run command needs a filename");
                }
            },
            Some("run") => {
                if let Some(file_name) = args.first() {
                    file_runner_handler(file_name);
                } else {
                    println!("run command needs a filename");
                }
            },
            Some("curl") => {
                if let Some(url) = args.first() {
                    request(url).await;
                } else {
                    println!("curl command needs a URL");
                }
            },
            Some("req") => {
                if let Some(url) = args.first() {
                    request(url).await;
                } else {
                    println!("curl command needs a URL");
                }
            },
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
    println!("exec - Runs a file");
    println!("run - Runs a file");  
    println!("curl - makes a GET request to a URL");
    println!("req - makes a GET request to a URL");
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

fn file_runner_handler(handler: &str) {
    let ext : Vec<&str> = handler.split(".").collect();
    if ext[1] == "sh" || ext[1] == "bash" {
        println!("Running Bash file... {}", handler);
        let out = process::Command::new("bash").arg(handler).output().expect("Running .sh/.bash file");
        print!("{}", String::from_utf8_lossy(&out.stdout))
    }
    else if ext[1] == "lua" {
        println!("Running Lua file... {}", handler);
        lua_handler(handler);
    }
    else {
        println!("Unknown file type. Supported files : .sh/.bash/.lua");
    }
}

fn lua_handler(handler: &str) {
    let file = fs::read(handler).unwrap();
    let lua_command = String::from_utf8(file).unwrap();
    let lua = Lua::new();
    let _ = lua.context(
        |lua_context|{
            lua_context.load(&lua_command).exec()
        }
    );
}

async fn request(url: &str) {
    if url.starts_with("http://") || url.starts_with("https://") {
        let requests = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("{:?}", requests)
    } else {
        println!("URL must start with http:// or https://");
    }
}
