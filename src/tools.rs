use std::io;

/// Generic function to ask user for input, trimed and lowercased.
pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    String::from(input.trim()).to_lowercase()
}

/// Generic function to position terminal to only show most recent information.
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
