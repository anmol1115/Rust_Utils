use std::{fs, os::unix::{ffi::OsStrExt, fs::PermissionsExt}};

use crate::Execute;
mod convert;

#[derive(Debug)]
pub struct Ls {
    path: String,
    long: bool,
    all: bool,
    help: bool
}

impl Ls {
    fn new(path: String) -> Ls {
        Ls {path, long: false, all: false, help: false}
    }
}

impl Execute for Ls {
    fn execute_command(&self) {
        if self.help {
            println!("Help message");
        } else {
            if let Ok(files) = fs::read_dir(&self.path) {
                for file in files {
                    if let Ok(file) = file {
                        let filename = file.file_name();
                        if filename.as_bytes()[0] as char != '.' || self.all {
                            if self.long {
                                if let Ok(metadata) = file.metadata() {
                                    let permissions = convert::numeric_to_symbolic_conv(metadata.permissions().mode());
                                    println!("{:?}      {:?}        {:?}", permissions, metadata.len(), filename);
                                }
                            } else {
                                println!("{:?}", filename);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn validate_args_for_ls(args: &Vec<String>) -> Result<Ls, String> {
    match args.len() {
        1 | 2 | 3 => {
            let path = args.last().unwrap().clone();
            let mut l = Ls::new(path);

            for idx in 0..args.len() - 1 {
                match args[idx].as_str() {
                    "-l" => {l.long = true},
                    "-a" => {l.all = true},
                    "-h" => {l.help = true},
                    _ => return Err("Invalid flag passed".to_string())
                }
            }
            Ok(l)
        },
        _ => Err("Invalid number of arguments passed".to_string())
    }
}