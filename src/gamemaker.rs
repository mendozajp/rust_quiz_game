use std::path::Path;

use rust_quiz_game::riddler::{self, load_single_exam_save_file, SavedQuiz};

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
    //     tools::clear_terminal();
    let mut game_state: GameState = match arg_file {
        Some(arg_file) => {
            let file_path: &Path = Path::new(&arg_file);
            let loaded_quiz: SavedQuiz = load_single_exam_save_file(file_path);
            single_examination(Some(loaded_quiz))
        }
        None => start_up_screen(),
    };
    loop {
        //     tools::clear_terminal();

        game_state = match game_state {
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
    match saved_quiz {
        None => {}
        Some(_) => {
            println!("We have a saved quiz!!");
            let quiz = saved_quiz.unwrap();
            println!("{:?}", quiz);
            return handle_user_action();
        }
    }
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
    if let Some(score) = riddler::Quiz::take_quiz(quiz.clone()) {
        riddler::Quiz::show_result(score, quiz.questions.len() as i32);
    } else {
        return GameState::QuitGame;
    }

    return handle_user_action();
}

fn game_show() -> GameState {
    println!("Appologies, this game mode has not been implemented yet.");
    return handle_user_action();
}
