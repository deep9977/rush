use std::collections::HashSet;
mod shell;
use shell::{Commands,Input};



fn main(){
    
    loop{
        shell::print_prompt();
        
        let tempinput = shell::read_command();
        match tempinput{
            Ok(_) =>{} ,
            Err("no input") => continue,
            _ =>{},
        }
        let input: Input = tempinput.unwrap();

        

        let mut commands: HashSet<&str> = HashSet::new();
        commands.insert("exit");
        commands.insert("echo");
        commands.insert("type");

        if commands.contains(input.command.as_str()){
            Commands::handle_command(&input);
        }
        else {
            println!("command not found: {}",input.command);
            continue;
        }
    
    }
}
