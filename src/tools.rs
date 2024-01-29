use std::io;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    String::from(input.trim()).to_lowercase()
}
