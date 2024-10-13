use std::fs::File;
use std::io;
use std::io::BufRead;
use std::{env, fs};

fn read_file(file_path: &String, flag: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    if flag == "-h" || flag == "--help" {
        println!("help");
    } else if flag == "-n" || flag == "--number" {
        for (index, line) in reader.lines().enumerate() {
            let content = line?;
            println!("{}: {}", index + 1, content);
        }
    } else if flag == "" {
        let content =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        println!("{content}");
    } else {
        println!("Invalid option {flag}");
        println!("Try neko --help for more information");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <program> [flag] <file_path>");
        return;
    }

    let flag: &str;
    let file_path: &String;

    if args.len() > 2 {
        flag = &args[1];
        file_path = &args[2];
    } else {
        flag = "";
        file_path = &args[1];
    }

    if let Err(e) = read_file(file_path, flag) {
        eprintln!("Error: {}", e);
    }
}
