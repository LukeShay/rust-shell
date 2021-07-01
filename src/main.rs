use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        prompt();

        let input = read_input();

        let (command, args) = split_input(&input);

        match command {
            "" => {},
            "cd" => cd(args),
            "clear" => clear(),
            "exit" => return,
            command => run_cmd(command, args),
        }
    }
}

/// Splits input into the command and the following arguments.
///
/// # Example
///
/// ```
/// let (command, args) = split_input(&String::new("command first_arg second_arg"));
/// ```
///
fn split_input(input: &str) -> (&str, std::str::SplitWhitespace) {
    let mut parts = input.trim().split_whitespace();
    (parts.next().unwrap_or_else(|| ""), parts)
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input
}

fn prompt() {
    let current_dir = env::current_dir().unwrap();
    let current_dir_value = current_dir.to_str().unwrap();
    
    let dir = match dirs::home_dir() {
        Some(home_dir) => {
            replace_home_dir(current_dir, home_dir)
        }
        None => {
            String::from(current_dir_value)
        }
    };

    print!("{} > ", dir);

    io::stdout().flush();
}

fn replace_home_dir(current_dir: std::path::PathBuf, home_dir: std::path::PathBuf) -> String {
    let current_dir_value = current_dir.to_str().unwrap();
    let home_dir_val = home_dir.to_str().unwrap();

    if current_dir_value.ne(home_dir_val) {
        current_dir_value.replace(home_dir_val, "~")
    } else {
        String::from(current_dir_value)
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H")
}

fn run_cmd(command: &str, args: std::str::SplitWhitespace) {
    match Command::new(command).args(args).spawn() {
        Ok(mut child) => {
            child.wait().expect("command wasn't running");
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn cd(args: std::str::SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}