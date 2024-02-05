use std::collections::HashMap;

use rust_quiz_game::riddler::{self, Question, Quiz, Quizes, SavedQuiz};
use std::io::Write;
use std::{fs, fs::File, path::Path};

fn create_test_question() -> Question {
    Question {
        question: "Test Question".to_string(),
        answer1: "Test Answer1".to_string(),
        answer2: "Test Answer2".to_string(),
        answer3: "Test Answer3".to_string(),
        answer4: "Test Answer4".to_string(),
        correct_answer: 1,
    }
}
fn create_test_quiz() -> Quiz {
    Quiz {
        quiz_name: "Test Quiz".to_string(),
        questions: HashMap::from([("question1".to_string(), create_test_question())]),
    }
}

fn create_test_quizes() -> Quizes {
    Quizes {
        available_quizes: vec![create_test_quiz()],
    }
}

fn create_saved_quiz() -> SavedQuiz {
    SavedQuiz {
        ordered_quiz: create_test_quiz(),
        answered_questions: Vec::new(),
    }
}

// Quizes Associated Function Tests
#[test]
fn test_display_quiz_names() {
    let test_quizes = create_test_quizes();
    test_quizes.display_quiz_names();
    println!("Confirm the following was printed just above: Test Quiz")
}

#[test]
fn test_read_quiz() {
    // these next 2 tests dont feel right. using unwraps like this
    // may seem clever but it feels like we are defeating the purpose of it.
    let test_quizes = create_test_quizes();

    // Successfully gets a quiz with the input quiz name
    assert_eq!(
        "Test Quiz",
        test_quizes
            .ready_quiz("Test Quiz".to_string().trim().to_lowercase()) // same logic as user_input
            .unwrap()
            .quiz_name,
    );
}

#[test]
#[should_panic]
fn test_read_quiz_1() {
    let test_quizes = create_test_quizes();

    // Fails to get a quiz with the input quiz name and returns none
    test_quizes.ready_quiz("Test Quiz1".to_string()).unwrap();
}

// Quiz Associated Function Tests
#[test]
fn test_get_quiz_length() {
    let test_quiz = create_test_quiz();

    assert_eq!(1, test_quiz.get_quiz_length());
}

/// Testing not panicing for operable range
#[test]
fn test_show_result() {
    for score in 0..100 {
        riddler::Quiz::show_result(score, 100)
    }
    for total_questions in 1..100 {
        riddler::Quiz::show_result(1, total_questions)
    }
}

/// Testing outside operable range
#[test]
#[should_panic]
fn test_show_result_1() {
    riddler::Quiz::show_result(11, 10)
}

#[test]
fn serde_integreation_testing() {
    let test_quiz = create_test_quiz();
    let test_answered_questions: Vec<(String, bool)> = vec![];

    // testing saving file
    assert_eq!(
        "Test_quiz.toml",
        riddler::create_and_save_single_exam_save_file(
            test_quiz,
            test_answered_questions,
            Some("Test_quiz.toml".to_string())
        )
        .unwrap(),
        "Failed to create save file",
    );

    // testing load saved file
    let test_saved_quiz = create_saved_quiz();
    let file_path: &Path = Path::new(&"Test_quiz.toml");

    assert_eq!(
        test_saved_quiz.ordered_quiz.quiz_name,
        riddler::load_single_exam_save_file(file_path)
            .ordered_quiz
            .quiz_name,
    );

    // clean up files created for test
    match fs::remove_file(file_path) {
        Ok(_) => (),
        Err(_) => println!("File generated for testing failed to be deleted. Be advised, you may have a new saved file named Test_quiz.toml")
    }
}

#[test]
fn test_load_quiz_from_toml() {
    //setup
    let file_name = "Test_quiz.toml";

    let saved_quiz = format!(
        r#"
quiz_name = "Test Quiz"

[questions.question1]
question = "Test Question"
answer1 = "Test Answer1"
answer2 = "Test Answer2"
answer3 = "Test Answer3"
answer4 = "Test Answer4"
correct_answer = 1
"#
    );
    // let saved_file: String = toml::to_string(&saved_quiz).expect("Failed to serialize toml file");
    let mut file = File::create(file_name).unwrap();
    match file.write(saved_quiz.as_bytes()) {
        Ok(_) => (),
        Err(_) => panic!(),
    }

    // start of test
    let file_path: &Path = Path::new(&"Test_quiz.toml");
    assert_eq!(
        "Test Quiz",
        riddler::load_quiz_from_toml(file_path).quiz_name
    );

    // clean up files created for test
    match fs::remove_file(file_name) {
        Ok(_) => (),
        Err(_) => println!("File generated for testing failed to be deleted. Be advised, you may have a new saved file named Test_quiz.toml")
    }
}
