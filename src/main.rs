use std::io::{stdin, stdout, Write};

use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush().ok();

        let mut input:String = String::new();

        stdin().read_line(&mut input).unwrap();

        // reading new line leaves a trailing new line.
        let command = input.trim();
        let mut child = Command::new(command)
          .spawn()
          .unwrap();

        child.wait().ok();
    }
  }
