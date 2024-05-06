use crate::Execute;

#[derive(Debug)]
pub struct Echo {
    message: String,
    to_uppercase: bool,
    to_lowercase: bool,
    help: bool
}

impl Echo {
    fn new(input: String) -> Echo {
        Echo {
            message: input, to_uppercase: false, to_lowercase: false, help: false
        }
    }
}

impl Execute for Echo {
    fn execute_command(&self) {
        if self.help {
            let message = r#"[USAGE]
echo [options...] message
    -l          print message in all lowercase
    -u          print message in all lowercase
    -h          print help for the command"#;
            println!("{message}");
        } else {
            if self.to_lowercase {
                println!("{}", self.message.to_lowercase());
            } else if self.to_uppercase {
                println!("{}", self.message.to_uppercase());
            } else {
                println!("{}", self.message);
            }
        }
    }
}

pub fn validate_args_for_echo(args: &Vec<String>) -> Result<Echo, String> {
    match args.len() {
        1 | 2 => {
            if args.len() == 1 {
                let output = args[0].clone();
                let echo = Echo::new(output);
                return Ok(echo)
            } else {
                match args[0].as_str() {
                    "-l" => {
                        let output = args[1].clone();
                        let mut echo = Echo::new(output);
                        echo.to_lowercase = true;
                        return Ok(echo)
                    },
                    "-u" => {
                        let output = args[1].clone();
                        let mut echo = Echo::new(output);
                        echo.to_uppercase = true;
                        return Ok(echo)
                    },
                    "-h" => {
                        let output = args[1].clone();
                        let mut echo = Echo::new(output);
                        echo.help = true;
                        return Ok(echo)
                    },
                    _ => return Err("Invalid flag passed".to_string())
                }
            }
        },
        _ => Err("Invalid number of arguments".to_string())
    }
}
