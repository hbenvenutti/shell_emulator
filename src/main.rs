use std::io::{stdin, stdout, Write};

use std::process::Command;

use std::env;
use std::path::Path;

fn main() {
  loop {
    print!("> ");
    stdout().flush().ok();

    let mut input:String = String::new();

    stdin().read_line(&mut input).unwrap();
    // reading new line leaves a trailing new line.

    let mut parts = input.trim().split_whitespace();

    let command = parts.next().unwrap();
    let args = parts;

    match command {
      "cd" => {
        let new_dir = args.peekable().peek().map_or("/", |x| *x);
        let root = Path::new(new_dir);
        
        if let Err(e) = env::set_current_dir(&root){
          eprintln!("{}", e);
        }
      },

      command => {
        let mut child = Command::new(command)
          .args(args)
          .spawn()
          .unwrap();

        child.wait().ok();
      }
    }
  }
}
