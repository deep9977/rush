
#[allow(unused_imports)]

use std::io::{self, Write , Error};
use std::process::exit;



enum Commands {
    Exit,
    Echo,
}
impl Commands{
    fn handle_command(command: Commands , args: &[String]){
        match command {
            Commands::Exit =>{
                exit(0);
            }
            Commands::Echo =>{
                println!("{}",args.join(" "));
            } 
           
        }
    }
}


fn main() {
    loop{
        print_prompt();
    // Wait for user input
        let input = read_command();
    
          if input.len() <= 1 {        
            continue;
        }
        let inputvector: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();
        let command = &inputvector[0];
        let args = &inputvector[1..];         

        match command.as_str() {
            "exit" => Commands::handle_command(Commands::Exit, args),
            "echo" => Commands::handle_command(Commands::Echo, args),
            _ => println!("{}: command not found",command.to_string()),
        }
    }

}


fn print_prompt(){
    let prompt_var = "$ ";
    print!("{}",prompt_var);
    io::stdout().flush().unwrap();
}
fn read_command() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut  input).unwrap();
    return input;
}

