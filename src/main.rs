use std::env;
use cli_utils::validate_args;

fn main() {
    let args: Vec<String> = env::args().collect();

    match validate_args(&args) {
        Ok(res) => res.execute_command(),
        Err(e) => println!("{e}")
    }
}
