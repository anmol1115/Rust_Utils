use std::fmt;

mod echo;
mod cat;
mod ls;
mod find;

enum Commands {
    Echo,
    Cat,
    Ls,
    Find,
    Invalid
}

pub fn validate_args(args: &Vec<String>) -> Result<Box<dyn Execute>, String> {
    match args.get(1) {
        Some(cmd) => {
            let cmd = get_command(cmd.clone().to_string());
            match cmd {
                Commands::Echo => {
                    let e = echo::validate_args_for_echo(&args[2..].to_vec())?;
                    return Ok(Box::new(e))
                },
                Commands::Cat => {
                    let c = cat::validate_args_for_cat(&args[2..].to_vec())?;
                    return Ok(Box::new(c))
                },
                Commands::Ls => {
                    let l = ls::validate_args_for_ls(&args[2..].to_vec())?;
                    return Ok(Box::new(l))
                },
                Commands::Find => {
                    let f = find::validate_args_for_find(&args[2..].to_vec())?;
                    return Ok(Box::new(f))
                }
                Commands::Invalid => Err("Invalid command passed".to_string())
            }
        },
        None => Err("Command not passed".to_string())
    }
}

fn get_command(cmd: String) -> Commands {
    match cmd.as_str() {
        "echo" => Commands::Echo,
        "cat" => Commands::Cat,
        "ls" => Commands::Ls,
        "find" => Commands::Find,
        _ => Commands::Invalid
    }
}

pub trait Execute: fmt::Debug {
    fn execute_command(&self);
}