use std::path::{Path, PathBuf};
use std::env;
mod riddler;
mod gamemaker;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
fn main() {
    // gamemaker::start_up_screen();
    let workdir = get_current_working_dir().unwrap();
    let workdir = workdir.display();
    println!("{workdir}");


    let test_quiz_url = Path::new("src/test_quiz.yaml");
    riddler::load_quiz_from_yaml(&test_quiz_url);
}
