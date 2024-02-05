use crate::tools;
use chrono::Local;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::{fs, fs::File, path::Path};
use toml;

/// Struct mainly for saving and loading files for single examination.
#[derive(Serialize, Deserialize, Clone)]
pub struct SavedQuiz {
    pub ordered_quiz: Quiz,
    pub answered_questions: Vec<(String, bool)>, // question name and if answered correctly.
}
impl SavedQuiz {
    /// handles skipping questions already answered from saved file.
    fn check_answered_question(
        question_number: &String,
        arr_of_answered_questions: &Vec<(String, bool)>,
    ) -> Option<bool> {
        for answered_question in arr_of_answered_questions {
            if question_number == &answered_question.0 {
                return Some(answered_question.1);
            }
        }
        return None;
    }
}

/// Struct for collecting and presenting all available quizes to the user for single examination
/// game mode.
#[derive(Clone)]
pub struct Quizes {
    pub available_quizes: Vec<Quiz>,
}

impl Quizes {
    /// Load all quiz toml files in quizes folder
    fn load_stored_quizes() -> Vec<Quiz> {
        let mut cached_quizes: Vec<Quiz> = Vec::new();
        let paths = fs::read_dir("src/quizes/").unwrap();

        for path in paths {
            cached_quizes.push(load_quiz_from_toml(&path.unwrap().path()));
        }
        cached_quizes
    }

    /// Creates quizes structure populated with all available quizes for user.
    pub fn setup_single_examination() -> Quizes {
        Quizes {
            available_quizes: Quizes::load_stored_quizes(),
        }
    }

    /// Display all quizes within quizes structure to user.
    pub fn display_quiz_names(&self) {
        for quiz in &self.available_quizes {
            println!("{}", quiz.quiz_name);
        }
    }

    /// Return a single quiz structure to begin game or none on invaild input.
    pub fn ready_quiz(self, input_quiz_name: String) -> Option<Quiz> {
        let mut quiz: Option<Quiz> = None;

        for single_quiz in self.available_quizes {
            if input_quiz_name == single_quiz.quiz_name.trim().to_lowercase() {
                quiz = Some(single_quiz.clone());
            }
        }

        return quiz;
    }
}

/// Main Structure for single examination, holds collection of questions for user to answer.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: HashMap<String, Question>,
}

impl Quiz {
    /// Goes through a series of prompts for user to answer all questions of a given quiz. Skips
    /// questions marked in answered_questions_record if loaded in via saved file.
    pub fn take_quiz(
        quiz: Quiz,
        answered_questions_record: Option<Vec<(String, bool)>>,
    ) -> Option<i32> {
        let mut score: i32 = 0;
        let mut loaded_saved_quiz: bool = false;
        let mut answered_questions_record: Vec<(String, bool)> = match answered_questions_record {
            None => Vec::new(),
            Some(answered_questions) => {
                loaded_saved_quiz = true;
                answered_questions
            }
        };
        let mut save_and_quit_prompt = false;

        // Cycle through questions
        for question in &quiz.questions {
            if loaded_saved_quiz {
                // TODO: obviously very ineffiecent, as if you loaded, this is growing throughout the test
                // ideally you would strip those and have the relevent information seperate?
                // Maybe we can solve with quiz metadata? Or clone og, strip down one for test taking and use
                // the og for results?
                match SavedQuiz::check_answered_question(question.0, &answered_questions_record) {
                    None => (),
                    Some(is_answer_correct) => {
                        if is_answer_correct {
                            score += 1;
                        }
                        continue; // skip question since it was answered
                    }
                }
            }
            match question.1.ask_question() {
                Some(true) => {
                    score += 1;
                    answered_questions_record.push((String::from(question.0), true));
                }
                Some(false) => {
                    answered_questions_record.push((String::from(question.0), false));
                    continue;
                }
                None => {
                    // only 'save and quit' input by user
                    save_and_quit_prompt = true;
                    break;
                }
            }
        }
        if save_and_quit_prompt {
            match create_and_save_single_exam_save_file(quiz, answered_questions_record, None) {
                Ok(file_name) => {
                    println!("Progess saved at {file_name}.");
                    return None;
                }
                Err(_) => {
                    println!("Something went wrong, save file cannot be generated.");
                    return None; // TODO: Loss of save file is pretty bad, find a way to cycle back into game or prompt user for choice.
                }
            };
        }
        Some(score)
    }

    /// return length of a quiz
    pub fn get_quiz_length(&self) -> i32 {
        return self.questions.len().clone() as i32;
    }

    /// report the outcome of a quiz given a score and the total number of questions in said quiz.
    pub fn show_result(score: i32, total_questions: i32) {
        tools::clear_terminal();
        let grade_number = score * 100 / total_questions;

        println!(
            "You scored {score} out of {} -- {grade_number}%",
            total_questions
        );
        //TODO: PLS get rid of this
        let grade: char;
        match grade_number {
            97..=100 => {
                grade = 'A';
                println!("Grade: {grade}+");
            }
            94..=96 => {
                grade = 'A';
                println!("Grade: {grade}");
            }
            90..=93 => {
                grade = 'A';
                println!("Grade: {grade}-");
            }
            87..=89 => {
                grade = 'B';
                println!("Grade: {grade}+");
            }
            84..=86 => {
                grade = 'B';
                println!("Grade: {grade}");
            }
            80..=83 => {
                grade = 'B';
                println!("Grade: {grade}-");
            }
            77..=79 => {
                grade = 'C';
                println!("Grade: {grade}+");
            }
            74..=76 => {
                grade = 'C';
                println!("Grade: {grade}");
            }
            70..=73 => {
                grade = 'C';
                println!("Grade: {grade}-");
            }
            67..=69 => {
                grade = 'D';
                println!("Grade: {grade}+");
            }
            64..=66 => {
                grade = 'D';
                println!("Grade: {grade}");
            }
            60..=63 => {
                grade = 'D';
                println!("Grade: {grade}-");
            }
            50..=59 => {
                grade = 'F';
                println!("Grade: {grade}");
            }
            0..=49 => {
                grade = 'F';
                println!("Grade: {grade}-");
            }
            _ => unreachable!(),
        }
        Quiz::_print_random_grade_message(grade);
    }

    fn _print_random_grade_message(grade: char) {
        let a = vec![
            "Fantastic work!",
            "Oh shit!",
            "Jesus!",
            "Fucking Hell!",
            "I'm not worthy of your presence!",
        ];
        let b = vec![
            "Nice.",
            "Could've been better.",
            "Alright, good job!",
            "Close enough I suppose.",
            "Nice work keeping above C level.",
        ];
        let c = vec![
            "Acceptable.",
            "Ok. Sure.",
            "Nothing special.",
            "Cs get degrees.",
        ];
        let d = vec![
            "Cutting it close eh?",
            "Bah, you'll get em next time.",
            "Hey, that's passing right?",
            "Do better.",
            "You got this. Never surrender. Give it another try.",
        ];
        let f = vec![
            "Damn, you fucking suck.",
            "Jesus man. Really?",
            "Were you even trying?",
            "Looks like all those brain cells really are gone.",
            "Sheeesh, nice work bro.",
            "Fucking dumb ass.",
            "Bruh",
        ];
        let mut rng = thread_rng();

        match grade {
            'A' => println!("{}", a[rng.gen_range(0..a.len() - 1)]),
            'B' => println!("{}", b[rng.gen_range(0..b.len() - 1)]),
            'C' => println!("{}", c[rng.gen_range(0..c.len() - 1)]),
            'D' => println!("{}", d[rng.gen_range(0..d.len() - 1)]),
            'F' => println!("{}", f[rng.gen_range(0..f.len() - 1)]),
            _ => {
                panic!("INVAILD GRADE")
            }
        }
    }
}

/// Struct for single question within a quiz. Standard multiple choice format.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub question: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub answer4: String,
    pub correct_answer: i8,
}

impl Question {
    fn ask_question(&self) -> Option<bool> {
        tools::clear_terminal();
        println!("{}", self.question);

        // shuffle order of answers to be displayed with an answer key
        // to reference later when checking correct answer
        let mut rng = rand::thread_rng();
        let mut shuffle_answer_key: Vec<i8> = (1..=4).collect();
        shuffle_answer_key.shuffle(&mut rng);

        let mut answers = Vec::new();
        for key in &shuffle_answer_key {
            match key {
                1 => answers.push(self.answer1.clone()),
                2 => answers.push(self.answer2.clone()),
                3 => answers.push(self.answer3.clone()),
                4 => answers.push(self.answer4.clone()),
                _ => unreachable!(),
            }
        }

        for answer in answers.iter().enumerate() {
            println!("[{}] {}", answer.0 + 1, answer.1);
        }
        println!("Enter the number next to the answer you beleive is correct.");
        println!("Type 'save and quit' if you would like to do so.");
        loop {
            let user_input = tools::read_input();

            if user_input == "save and quit" {
                println!("Beginning save and quit functionality...");
                return None; // begin generating save file.
            }
            let user_answer: usize = match user_input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(
                        "Invaild input, please enter your guess by typing its corresponding number"
                    );
                    continue;
                }
            };
            if user_answer > 4 || user_answer < 1 {
                println!("Invaild input, please enter one of the available answers.");
                continue;
            }
            println!();
            if shuffle_answer_key[user_answer - 1] == self.correct_answer {
                return Some(true);
            } else {
                return Some(false);
            }
        }
    }
}

/// Load a toml quiz file into memory
/// TODO: Add readability for multiple quizes in one file
pub fn load_quiz_from_toml(path: &Path) -> Quiz {
    let toml_str = fs::read_to_string(path).expect("Failed to read toml file");
    let quiz: Quiz = toml::from_str(&toml_str).expect("Failed to deserialize toml file");

    return quiz;
}

/// Load a toml quiz file into memory
pub fn load_single_exam_save_file(path: &Path) -> SavedQuiz {
    let toml_str = fs::read_to_string(path).expect("Failed to read toml file");
    let saved_quiz: SavedQuiz = toml::from_str(&toml_str).expect("Failed to deserialize toml file");

    return saved_quiz;
}

/// Creates and saves a saved file from the single examination game mode to project dir
pub fn create_and_save_single_exam_save_file(
    quiz_to_save: Quiz,
    answered_questions: Vec<(String, bool)>,
    name_of_save_file: Option<String>, // TODO: Currently only for unit tests
) -> std::io::Result<String> {
    let saved_quiz = SavedQuiz {
        ordered_quiz: quiz_to_save,
        answered_questions: answered_questions,
    };
    let saved_file: String = toml::to_string(&saved_quiz).expect("Failed to serialize toml file");
    let file_name = match name_of_save_file {
        None => {
            format!(
                "{}_single_exam_save_file.toml",
                Local::now().format("%d-%m-%Y_%H:%M")
            )
        }
        Some(file_name) => file_name,
    };

    let mut file = File::create(file_name.clone())?;
    file.write(saved_file.as_bytes())?;
    Ok(file_name)
}

/// testing private functions
#[cfg(test)]
mod tests {
    use crate::riddler;
    #[test]
    fn test_check_answered_questions() {
        let question_number = "question1".to_string();
        let arr_of_answered_questions1 = vec![("question1".to_string(), false)];
        let arr_of_answered_questions2 = vec![("question2".to_string(), false)];

        assert_eq!(
            Some(false),
            riddler::SavedQuiz::check_answered_question(
                &question_number,
                &arr_of_answered_questions1
            )
        );
        assert_eq!(
            None,
            riddler::SavedQuiz::check_answered_question(
                &question_number,
                &arr_of_answered_questions2
            )
        );
    }
    #[test]
    fn test_print_random_grade_message() {
        riddler::Quiz::_print_random_grade_message('A');
        riddler::Quiz::_print_random_grade_message('B');
        riddler::Quiz::_print_random_grade_message('C');
        riddler::Quiz::_print_random_grade_message('D');
        riddler::Quiz::_print_random_grade_message('F');
    }

    #[test]
    #[should_panic]
    fn test_print_random_grade_message_1() {
        riddler::Quiz::_print_random_grade_message('S');
    }
}
