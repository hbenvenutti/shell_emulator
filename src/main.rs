use std::io::{stdin, stdout, Write};

use std::process::{Child, Command, Stdio};

use std::env;
use std::path::Path;

fn main() {
  loop {
    print!("> ");
    stdout().flush().ok();
  
    let mut input:String = String::new();
  
    stdin().read_line(&mut input).unwrap();
    // reading new line leaves a trailing new line.
  
    
    let mut commands = input.trim().split("|").peekable();
    let mut previous_command = None;
  
    while let Some(command) = commands.next() {
  
      // -- Argumentos ---------------------------------------------------------------------------- //
      // Divide o input em comando e argumentos
      let mut parts = command.trim().split_whitespace();
  
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

          previous_command = None;
        },
        
        // -- Fechar o terminal ------------------------------------------------------------------- //
        "exit" => return,
  
        // -- Execução dos comandos --------------------------------------------------------------- //
        command => {
          let stdin = previous_command
            .map_or(
              Stdio::inherit(), 
              |output: Child| Stdio::from(output.stdout.unwrap())
            );
  
          let stdout = if commands.peek().is_some() {
              Stdio::piped()
          } else {
              Stdio::inherit() // por algum motivo ";" faz o código não rodar...
          };
  
          let output = Command::new(command)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn();
  
          match output {
            Ok(output) => {previous_command = Some(output);},
            Err(e) => {
              previous_command = None;
              eprintln!("{}", e);
            }
          }
        }
      }    
    }
    if let Some(mut final_command) = previous_command {
      final_command.wait().unwrap();
    }
  }
}
