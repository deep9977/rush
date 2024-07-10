use std::{io::{self, Write}, process::Command};
use core::result::Result;
use std::process::exit;


pub enum Commands {
    Exit ,
    Echo ,
    Type ,
    NotFound,
}
impl Commands{
    fn what_command(input: &String) -> Commands{
        match input.as_str(){
            "exit" => Commands::Exit,
            "echo" => Commands::Echo,
            "type" => Commands::Type,
            _ => Commands::NotFound,
        }
    }
    pub fn handle_command(input: &Input) { 
        let whatcommand: Commands = Self::what_command(&input.command);
        
        match whatcommand {
            Commands::Exit => exit(0) ,
            Commands::Echo => Self::echo(&input) ,
            Commands::Type => Self::typefn(&input) ,
            _ => {},
        }
    }
    fn echo(input: &Input){
        println!("{}",input.args.join(" "));
    }
    fn typefn(input: &Input){
        if !matches!(Self::what_command(&input.args[0]) , Commands::NotFound){
            println!("{} is shell builtin",input.args[0].as_str());
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
