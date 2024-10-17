use std::fs::File;
use std::io;
use std::io::BufRead;
use std::{env, fs};

fn read_file(file_path: &String, flag: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    if flag == "-h" || flag == "--help" {
        println!("Usage: neko [flag] <file_path>");
        println!("Concatenate FILE(s) to standard output.");
        println!(
            "
With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help        display this help and exit
      --version     output version information and exit

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.
    "
        );
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

    //if args.len() < 2 {
    //    eprintln!("Usage: neko [flag] <file_path>");
    //    return;
    //}

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
