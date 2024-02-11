use std::env;
use cli_utils::validate_args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let res = validate_args(&args).expect("msg");
    res.execute_command();
}
