use std::env;

use cli_utils::validate_args;
use cli_utils::Execute;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let res = validate_args(&args).expect("msg");
    res.execute_command();
}
