use std::{fs, io};

#[path = "riddler.rs"]
mod riddler;

/// framing enum for the whole project, you will at all times be in one of these states. Make sure
/// the code reflext that.
enum GameState {
    StartUpScreen,
    SingleExamination,
    GameShow,
    SaveAndQuit, // TODO: thinking about it now, we need a seperate way to do this since handle
    // user action is blocking, we cant do that during a quiz. We could make
    // handle user action take an optional and just run it with that.
    QuitGame,
}

/// Main transition state for the player.
/// Starts and stops all states of the game.
fn handle_user_action() -> GameState {
    loop {
        println!("Please type one of the following game modes or type 'exit' to quit.");
        println!("Start up Screen");
        println!("Single Examination");
        println!("Game Show");

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let user_action = String::from(action.trim()).to_lowercase();

        println!();
        match user_action.as_str() {
            "exit" => return GameState::QuitGame,
            "start up screen" => return GameState::StartUpScreen,
            "single examination" => return GameState::SingleExamination,
            "game show" => return GameState::GameShow,
            "save and quit" => return GameState::SaveAndQuit,
            _ => println!("not a valid action, please enter one of the game modes as displayed."),
        }
    }
}

// main loop for switching between game states
pub fn main_loop() {
    let mut game_state: GameState = GameState::StartUpScreen;
    // add load save here if available
    loop {
        // should have a shell reset here to keep things clean
        match game_state {
            GameState::StartUpScreen => game_state = start_up_screen(),
            GameState::SingleExamination => game_state = single_examination(),
            GameState::GameShow => game_state = game_show(),
            GameState::SaveAndQuit => game_state = save_and_quit(),
            GameState::QuitGame => break,
        }
    }
    println!("Thank you for playing!");
}

// Game State - Start up Screen
fn start_up_screen() -> GameState {
    println!("Welcome To Quiz Show!\n");
    return handle_user_action();
}

/// Game state - Single Examination
/// Guides user through quiz, prompts for every question and returns result upon completion.
fn single_examination() -> GameState {
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
    println!("Appologies, this game mode has not been implemented yet.");
    return handle_user_action();
}

/// Load all quiz toml files in quizes folder
fn load_stored_quizes() -> Vec<riddler::Quiz> {
    let mut cached_quizes: Vec<riddler::Quiz> = Vec::new();
    let paths = fs::read_dir("src/quizes/").unwrap();

    for path in paths {
        cached_quizes.push(riddler::load_quiz_from_toml(&path.unwrap().path()));
    }
    cached_quizes
}

fn game_show() -> GameState {
    println!("Appologies, this game mode has not been implemented yet.");
    return handle_user_action();
}

fn save_and_quit() -> GameState {
    println!("Appologies, this game mode has not been implemented yet.");
    return handle_user_action();
}
