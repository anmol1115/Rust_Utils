use std::fs;

use crate::Execute;

#[derive(Debug)]
pub struct Find {
    path: String,
    query: String,
    file_type: Option<String>,
    help: bool
}

impl Find {
    pub fn new(path: String, query: String) -> Find {
        Find{path, query, file_type: None, help: false}
    }
}

impl Execute for Find {
    fn execute_command(&self) {
        if self.help {
            let message = r#"[USAGE]
find [options...] path/to/dir expression
    -t type     search for specific file type. Allowed values 'file' and 'dir'
    -h          print help for the command"#;
            println!("{message}");
        } else {
            let mut res = Vec::new();

            if let Ok(files) = fs::read_dir(&self.path) {
                for file in files {
                    if let Ok(file) = file {
                        if let Some(file_type) = &self.file_type {
                            if file_type == "file" {
                                if let Ok(metadata) = file.metadata() {
                                    if metadata.is_dir() {
                                        continue
                                    }
                                }
                            }
                            if file_type == "dir" {
                                if let Ok(metadata) = file.metadata() {
                                    if metadata.is_file() {
                                        continue
                                    }
                                }
                            }
                        }
                        if file.file_name().into_string().unwrap().contains(&self.query) {
                            res.push(file.file_name())
                        }
                    }
                }
            }

            for entry in res {
                println!("{}/{}", &self.path, entry.into_string().unwrap());
            }
        }
    }
}

pub fn validate_args_for_find(args: &Vec<String>) -> Result<Find, String> {
    match args.len() {
        2 | 3 | 4  => {
            let query = args.last().unwrap().clone();
            let path = args.get(args.len()-2).unwrap().clone();
            let mut f = Find::new(path, query);

            let mut idx = 0;
            while idx < args.len()-2 {
                match args[idx].as_str() {
                    "-t" => {
                        idx += 1;
                        match args[idx].as_str() {
                            "file" => {f.file_type = Some("file".to_string())},
                            "dir" => {f.file_type = Some("dir".to_string())},
                            _ => return Err("Invalid file type".to_string())
                        }
                    },
                    "-h" => {f.help = true},
                    _ => return Err("Invalid flag passed".to_string())
                }
                idx += 1;
            }

            Ok(f)
        },
        _ => Err("Invalid number of arguments".to_string())
    }
}