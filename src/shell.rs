use std::io::{self, Write};
use core::result::Result;
use std::process::exit;
use std::env;
use std::fs;

pub enum Commands {
    Exit ,
    Echo ,
    Type ,
    ExternCommand ,
    NotFound,
}
impl Commands{
    fn what_command(input: &String) -> Commands{
        match input.as_str(){
            "exit" => Commands::Exit,
            "echo" => Commands::Echo,
            "type" => Commands::Type,
            _ => {
                if let Ok(_) = Self::handle_external_commmand(&input){
                    Commands::ExternCommand
                }else{
                    Commands::NotFound
                }
            }
        }
    }
    fn handle_external_commmand(command: &String) -> Result<String , &'static str>{
        let path = env::var("PATH").unwrap();
        let path_s: Vec<&str> = path.split(":").collect();
        let mut found = false;

        for p in path_s.iter(){
            let mut str1: String = p.to_string().clone();
            str1.push('/');
            str1.push_str(command.as_str());
            if fs::metadata(&str1).is_ok(){
                found = true;
                return Ok(str1);
            }
        
        }if !found{
            return Err("bin not found");
        }

        Err("Unexpected Error Occurred ")
    }


    pub fn handle_command(input: &Input) { 
        let whatcommand: Commands = Self::what_command(&input.command);
        
        match whatcommand {
            Commands::Exit => exit(0) ,
            Commands::Echo => Self::echo(&input) ,
            Commands::Type => Self::typefn(&input) ,
            Commands::ExternCommand => println!("external command "),
            _ => println!("command not found: {}",input.command),
        }
    }
    fn echo(input: &Input){
        println!("{}",input.args.join(" "));
    }
    fn typefn(input: &Input){
        if !matches!(Self::what_command(&input.args[0]) , Commands::NotFound){
            println!("{} is shell builtin",input.args[0].as_str());
        }else if let Ok(path) = Self::handle_external_commmand(&input.args[0].to_string()){
            println!("{} is {}",input.args[0].as_str(), path);
        }else{
            println!("{} not found",input.args[0].as_str());
        }
    }
}


pub struct Input{
    pub command: String, 
    pub args: Vec<String>,
}
pub fn print_prompt(){
    let prompt_var = "$ ";
    print!("{}",prompt_var);
    io::stdout().flush().unwrap();
}
pub fn read_command() -> Result<Input , &'static str>{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    
    let mut command_vector: Vec<String> = user_input.trim().split_ascii_whitespace().map(|s|s.to_string()).collect();

    if command_vector.is_empty(){
        Err("no input")
    }else{
        let user_command = command_vector.remove(0);
        let args: Vec<String> = command_vector;
    
        let input = Input{
            command: user_command.clone(),
            args: args,
        };
        Ok(input)
    
    
    }    
}
