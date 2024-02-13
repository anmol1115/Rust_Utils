use std::fs;
use crate::Execute;

#[derive(Debug)]
pub struct Grep {
    path: String,
    query: String,
    case: bool,
    help: bool
}

impl Grep {
    pub fn new(path: String, query: String) -> Grep {
        Grep {path, query, case: false, help: false}
    }
}

impl Execute for Grep {
    fn execute_command(&self) {
        if self.help {
            let message = r#"[USAGE]
grep [options...] path/to/dir expression
    -c          Turn case sensitivity OFF. Default is ON
    -h          print help for the message"#;
            println!("{message}");
        } else {
            if let Ok(content) = fs::read_to_string(&self.path) {
                for line in content.split('\n') {
                    if line.contains(&self.query) || (self.case && line.to_lowercase().contains(&self.query.to_lowercase())) {
                        println!("{line}");
                    }
                }
            }
        }
    }
}

pub fn validate_args_for_grep(args: &Vec<String>) -> Result<Grep, String> {
    match args.len() {
        2 | 3 | 4 => {
            let query = args.last().unwrap().clone();
            let path = args.get(args.len()-2).unwrap().clone();
            let mut g = Grep::new(path, query);

            let mut idx = 0;
            while idx < args.len()-2 {
                match args[idx].as_str() {
                    "-h" => {g.help = true},
                    "-c" => {g.case = true},
                    _ => return Err("Invalid flag passed".to_string())
                }
                idx += 1;
            }

            Ok(g)
        }
        _ => Err("Invalid number of arguments".to_string())
    }
}