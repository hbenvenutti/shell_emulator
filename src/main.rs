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

    // -- Argumentos ---------------------------------------------------------------------------- //
    // Divide o input em comando e argumentos
    let mut parts = input.trim().split_whitespace();

    let command = parts.next().unwrap();
    let args = parts;


    match command {      
      // -- Builtin cd -------------------------------------------------------------------------- //
      "cd" => {
        let new_dir = args.peekable().peek().map_or("/", |x| *x);
        // executar cd sem especificar um diretório muda para o '/'.

        let root = Path::new(new_dir);
        
        if let Err(e) = env::set_current_dir(&root){
          eprintln!("{}", e);
        }
      },

      // -- Fechar o terminal ------------------------------------------------------------------- //
      "exit" => return,

      // -- Execução dos comandos --------------------------------------------------------------- //
      command => {
        let mut child = Command::new(command)
          .args(args)
          .spawn()
          .unwrap();

        child.wait().ok(); // Não deixa dois comandos executarem ao mesmo tempo.
      }
    }
  }
}
