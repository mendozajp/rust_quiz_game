use std::path::Path;
use crate::riddler;
use crate::tools;

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
    tools::clear_terminal();
    let mut game_state: GameState = match arg_file {
        Some(arg_file) => {
            let file_path: &Path = Path::new(&arg_file);

            let loaded_quiz: riddler::Quiz = match riddler::Quiz::load(file_path) {
                Ok(saved_quiz) => saved_quiz,
                Err(e) => {
                    println!("Encountered errors while loading saved file: \n{e}");
                    println!(
                        "Something may be wrong with the format of the file, rendering it useless."
                    );
                    println!("Please start the program again without the file as an arguement.");
                    return;
                }
            };
            single_examination(Some(loaded_quiz))
        }
        None => start_up_screen(),
    };
    loop {
        tools::clear_terminal();

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
    println!("To load a save file, please input the file name of a saved file as an arguement when starting the quiz game.");
    handle_user_action()
}

/// Game state - Single Examination
/// Guides user through quiz, prompts for every question and returns result upon completion.
fn single_examination(saved_quiz: Option<riddler::Quiz>) -> GameState {

    let quiz: Option<riddler::Quiz> = match saved_quiz {
        None => {
            let quizes = match riddler::QuizList::load_stored_quizes() {
                Ok(quizes) => quizes,
                Err(e) => {
                    println!("Error on loading stored quizes: {e}");
                    return GameState::StartUpScreen; // leads to a reset so you dont end up seeing that error
                }
            };
            println!("Quizes available for testing:");
            println!("{quizes}");
            prompt_for_quiz() // can return none if user returns to start up screen or error on loading quizes
        }
        Some(saved_quiz) => {
            Some(saved_quiz)
        }
    };

    // catch user prompt to return to start up screen or unwrap
    let quiz = match quiz {
        None => return GameState::StartUpScreen,
        Some(quiz) => quiz,
    };

    if let Some(quiz) = quiz.begin_quiz() {
        quiz.show_result();
    } else {
        // saving and quiting returns none, thus quiting the game after logic for saving state
        return GameState::QuitGame;
    }

    handle_user_action()
}

fn game_show() -> GameState {
    println!("Apologies, this game mode has not been implemented yet.");
    handle_user_action()
}

fn prompt_for_quiz() -> Option<riddler::Quiz> {
    let selected_quiz: Option<riddler::Quiz>;

    loop {
        let quizes = match riddler::QuizList::load_stored_quizes() {
            Ok(quizes) => quizes,
            Err(e) => {
                println!("Error on loading stored quizes: {e}");
                println!("Returning to startup screen");
                return None;
            }
        };
        println!("Please enter one of the above displayed quizes to start, or return by entering 'start up screen'");
        let user_input = tools::read_input();

        if user_input == "start up screen" {
            return None;
        }

        let user_selected_quiz = quizes.ready_quiz(user_input);
        if user_selected_quiz.is_none() {
            println!("Quiz not available, confirm spelling.");
            continue;
        }
        selected_quiz = user_selected_quiz;
        break;
    }
    selected_quiz
}
