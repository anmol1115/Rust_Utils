use std::fs;
use std::cmp::{min, max};

use crate::Execute;

#[derive(Debug)]
pub struct Cat {
    content: String,
    head: Option<usize>,
    tail: Option<usize>,
    help: bool
}

impl Cat {
    fn new(content: String) -> Cat {
        Cat {content, head: None, tail: None, help: false}
    }
}

impl Execute for Cat {
    fn execute_command(&self) {
        if self.help {
            let message = r#"[USAGE]
cat [options...] message
    -a count    print 'count' number of lines from the top
    -t count    print 'count' number of lines from the bottom
    -h          print help for the command"#;
            println!("{message}");
        } else {
            let content: Vec<&str> = self.content.split('\n').collect();

            if let Some(count) = self.head {
                for idx in 0..min(count, content.len()) {
                    println!("{}", content[idx]);
                }
                return;
            }

            if let Some(count) = self.tail {
                for idx in max(content.len()-count,0)..content.len() {
                    println!("{}", content[idx]);
                }
                return;
            }

            for idx in 0..content.len() {
                println!("{}", content[idx]);
            }
        }
    }
}

pub fn validate_args_for_cat(args: &Vec<String>) -> Result<Cat, String> {
    match args.len() {
        1 | 2 | 3 => {
            let path = args.last().unwrap().clone();
            let mut c = match fs::read_to_string(path) {
                Ok(contents) => Cat::new(contents),
                Err(e) => return Err(e.to_string())
            };

            let mut idx = 0;
            while idx < args.len()-1 {
                let flag = args[idx].as_str();
                match flag {
                    "-a" => {
                        let count = match args[idx+1].parse::<usize>() {
                            Ok(num) => {
                                idx += 1;
                                num
                            },
                            Err(e) => return Err(e.to_string())
                        };
                        c.head = Some(count);
                    },
                    "-t" => {
                        let count = match args[idx+1].parse::<usize>() {
                            Ok(num) => {
                                idx += 1;
                                num
                            },
                            Err(e) => return Err(e.to_string())
                        };
                        c.tail = Some(count);
                    },
                    "-h" => {c.help = true},
                    _ => return Err("Invalid flag passed".to_string())
                };
                idx += 1;
            }

            Ok(c)
        }
        _ => Err("Invalid number of arguments".to_string())
    }
}