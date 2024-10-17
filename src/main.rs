use std::env;
use std::fs::File;
use std::io::BufRead;
use std::{io, process};

fn print_help() {
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
}

fn print_lines<I>(lines: I, flag: &str) -> io::Result<()>
where
    I: Iterator<Item = io::Result<String>>,
{
    match flag {
        "-n" | "--number" => {
            for (index, line) in lines.enumerate() {
                let content = line?;
                println!("{}: {}", index + 1, content);
            }
        }
        "" => {
            for line in lines {
                println!("{}", line?);
            }
        }
        _ => {
            eprintln!("neko: Invalid option : '{flag}'");
            eprintln!("try 'neko --help' for more information.");
            process::exit(1);
        }
    }
    Ok(())
}

fn read_file(file_path: &String, flag: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines = reader.lines();

    let _ = print_lines(lines, flag);
    Ok(())
}

fn read_from_stdin(flag: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock().lines();

    print_lines(reader, flag)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let flag: Option<&str>;
    let file_path: Option<&String>;

    match args.len() {
        1 => {
            if let Err(e) = read_from_stdin("") {
                eprintln!("Error reading from stdin: {}", e);
            }
            return;
        }
        2 => {
            if args[1].starts_with("-") {
                flag = Some(&args[1]);
                file_path = None;
                _ = file_path;
                if let Some(flag) = flag {
                    if flag == "-h" || flag == "--help" {
                        print_help();
                    } else {
                        eprintln!("Usage: neko [flag] [file_path]");
                        process::exit(1);
                    }
                }
                println!("Flag provided: {}", flag.unwrap());
            } else {
                flag = None;
                _ = flag;
                file_path = Some(&args[1]);

                if file_path.unwrap().is_empty() {
                    eprintln!("Error: file path is empty");
                    return;
                }

                if let Err(e) = read_file(file_path.unwrap(), "") {
                    eprintln!("Error: {}", e);
                }
            }
        }
        3 => {
            flag = Some(&args[1]);
            file_path = Some(&args[2]);

            if let Err(e) = read_file(file_path.unwrap(), flag.unwrap()) {
                eprintln!("Error: {}", e);
            }
        }
        _ => {
            eprintln!("Usage: neko [flag] [file_path]");
            process::exit(1);
        }
    }
}
