use std::collections::HashSet;
mod shell;
use shell::{Commands,Input};



fn main(){
    
    loop{
        shell::print_prompt();

        let input: Input;
        if let Ok(tempinput) = shell::read_command(){
            input = tempinput;
        }else{
            continue;
        }

        let mut commands: HashSet<&str> = HashSet::new();
        commands.insert("exit");
        commands.insert("echo");
        commands.insert("type");


        Commands::handle_command(&input);

    }
}
