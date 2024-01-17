/// framing enum for the whole project, you will at all times be in one of these states. Make sure
/// the code reflext that.
enum Game_State {
    startUpScreen,
    singleExamination,
    gameShow,
    saveAndQuit,
}

/// Main transition state for the player.
/// Starts and stops all states of the game.
pub fn handle_user_action(action: Game_State) {}

// Game State - Start up Screen
fn start_up_screen() {
    println!("Welcome To Quiz Show!\n");
    println!("Please type one of the following game modes or type 'exit' to quit.");
    println!("Single Examination");
    println!("Game Show(not implemented yet)");

    let mut action = String::new();

    loop {
        // until they type a quiz name or exit
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
    }
}

/// Game state - Single Examination
/// Guides user through quiz, prompts for every question and returns result upon completion.
fn single_examination() {
    fn prompt_user_for_quiz() {}
    fn take_quiz() {} // if display impl works out, we can have it all in there anyways.
    fn show_result() {}

    /// Should return random message from pool of corresponding grade.
    /// I say have 10 of each if you can manage it.
    /// also still not sure how we are gonna manage displaying messages from here.
    /// but i think enum examples will give us what we need.
    enum result_message_manager {
        Grade_A,
        Grade_B,
        Grade_C,
        Grade_D,
        Grade_F,
    }
}

/// Load all quiz toml files in quizes folder
pub fn load_stored_quizes() -> Vec<riddler::Quiz> {
    let mut cached_quizes: Vec<Quiz> = Vec::new();
    let paths = fs::read_dir("src/quizes/").unwrap();

    for path in paths {
        cached_quizes.push(riddler::load_quiz_from_toml(&path.unwrap().path()));
    }
    cached_quizes
}

pub fn prompt_for_continued_action() {}

pub fn save_and_quit() {}
