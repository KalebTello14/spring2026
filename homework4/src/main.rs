use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation{
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                eprintln!("Error listing directory");
            }
        }
        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                eprintln!("Error displaying file");
            }
        }
        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > '{}'", content, file);
            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File created succuessfully.");
            } else {
                eprintln!("Failed to create file.");
            }
        }
        FileOperation::Remove(file) => {
            let status = Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File removed succuessfully.");
            } else {
                eprintln!("Failed to remove file.");
            }
        }
        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                eprintln!("Error retrieving working directory");
            }
        }
    }  
}
fn read_input() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

fn main(){
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("1. Display file contents");
        println!("1. create a new file");
        println!("1. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("\nEnter your choice (0-5):");

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                println!("Enter directory path:");
                let path = read_input();

                let op = FileOperation::List(path);
                perform_operation(op);
            }

            "2" => {
                println!("Enter file path:");
                let file = read_input();

                let op = FileOperation::Display(file);
                perform_operation(op);
            }

            "3" => {
                println!("Enter file path:");
                let file = read_input();

                println!("Enter content:");
                let content = read_input();

                let op = FileOperation::Create(file, content);
                perform_operation(op);
            }

            "4" => {
                println!("Enter file path:");
                let file = read_input();

                let op = FileOperation::Remove(file);
                perform_operation(op);
            }

            "5" => {
                let op = FileOperation::Pwd;
                perform_operation(op);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please choose between 0 and 5.");
            }
        }
    }
}