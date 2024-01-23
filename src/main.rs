use std::path::Path;

mod gamemaker;
mod gamemakerv2;
mod riddler;

fn main() {
    // gamemaker::start_up_screen();
    // let file_path: &Path = Path::new("src/test_quiz.toml");
    // riddler::load_quiz_from_toml(file_path);
    gamemakerv2::main_loop();
}
