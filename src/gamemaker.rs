use std::path::Path;

use rust_quiz_game::riddler::{self, SavedQuiz};

#[path = "tools.rs"]
mod tools;

/// framing enum for the whole game, at all times you will be in one of these states.
enum GameState {
    StartUpScreen,
    SingleExamination,
    GameShow,
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

        let user_action = tools::read_input();

        println!();
        match user_action.as_str() {
            "exit" => return GameState::QuitGame,
            "start up screen" => return GameState::StartUpScreen,
            "single examination" => return GameState::SingleExamination,
            "game show" => return GameState::GameShow,
            _ => println!("not a valid action, please enter one of the game modes as displayed."),
        }
    }
}

// main loop for switching between game states
pub fn main_loop(arg_file: Option<String>) {
    let game_state: GameState = match arg_file {
        Some(String) => single_examination(Some(saved_quiz)),
        None => start_up_screen(),
    };
    // add load save here if available
    loop {
        // should have a shell reset here to keep things clean
        for _ in 0..3 {
            println!(); // TODO: replace with bash reset
        }
        let game_state: GameState = match game_state {
            GameState::StartUpScreen => start_up_screen(),
            GameState::SingleExamination => single_examination(None),
            GameState::GameShow => game_show(),
            GameState::QuitGame => break,
        };
    }
    println!("Thank you for playing!");
}

/// Game State - Start up Screen
/// Currently doesnt do anything but welcome user to game.
fn start_up_screen() -> GameState {
    println!("Welcome To Quiz Show!\n");
    return handle_user_action();
}

/// Game state - Single Examination
/// Guides user through quiz, prompts for every question and returns result upon completion.
fn single_examination(saved_quiz: Option<SavedQuiz>) -> GameState {
    let quizes = riddler::Quizes::setup_single_examination();
    let mut quiz: Option<riddler::Quiz>;

    println!("Quizes available for testing:");
    quizes.display_quiz_names();

    loop {
        let quizes = riddler::Quizes::setup_single_examination();
        println!("Please enter one of the above displayed quizes to start, or return by entering 'start up screen'");
        let user_input = tools::read_input();

        if user_input == "start up screen" {
            return GameState::StartUpScreen;
        }

        quiz = quizes.ready_quiz(user_input);
        if quiz.is_none() {
            println!("Quiz not available, confirm spelling.");
            continue;
        }
        break;
    }

    let quiz = quiz.unwrap();
    riddler::Quiz::show_result(quiz.clone().take_quiz(), quiz.questions.len() as i32);

    return handle_user_action();
}

fn game_show() -> GameState {
    println!("Appologies, this game mode has not been implemented yet.");
    return handle_user_action();
}
