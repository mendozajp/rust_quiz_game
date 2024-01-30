mod gamemaker;

use std::{env, io};
fn main() {
    let args: Vec<String> = env::args().collect();

    match read_args(&args) {
        Ok(None) => gamemaker::main_loop(None),
        Ok(any_string) => gamemaker::main_loop(any_string),
        Err(e) => {
            println!("{e}");
        }
    }

    dbg!(&args[1]);
}

fn read_args(args: &[String]) -> Result<Option<String>, String> {
    if args.len() == 1 {
        Ok(None)
    } else if args.len() == 2 {
        return Ok(Some(args[1].clone()));
    } else {
        return Err("To many arguments, not supported.".to_string());
    }
}
