use std::io::stdin;
use std::process::Command;

fn main() {
    let mut input:String = String::new();
    
    stdin().read_line(&mut input).unwrap();
  
    // reading new line leaves a trailing new line.
  
    let command = input.trim();
  
    Command::new(command)
      .spawn()
      .unwrap();
  }
