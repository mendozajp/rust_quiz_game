use anyhow::Result;
use chrono::Local;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::{fmt, path::Path};
use std::{fs, fs::File};

use crate::tools;

/// Struct mainly for saving and loading files for single examination.
#[derive(Serialize, Deserialize, Clone)]
pub struct SavedQuiz {
    pub quiz_in_progress: Quiz,
    pub answered_questions: HashMap<String, bool>, // question name and if answered correctly.
}

impl SavedQuiz {
    fn check_answered_question(
        question_name: &String,
        arr_of_answered_questions: &HashMap<String, bool>,
    ) -> Option<bool> {
        for answered_question in arr_of_answered_questions {
            if &question_name == &answered_question.0 {
                return Some(*answered_question.1);
            }
        }
        None
    }

    /// Load a toml quiz file into memory
    pub fn load(path: &Path) -> Result<SavedQuiz> {
        let toml_str = fs::read_to_string(path)?;
        let saved_quiz: SavedQuiz = toml::from_str(&toml_str)?;
        Ok(saved_quiz)
    }

    /// saves to specific path
    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        let saved_file: String = toml::to_string(&self)?;
        let mut file = File::create(path).expect("failed to create file");
        file.write_all(saved_file.as_bytes())
            .expect("failed to write all");
        Ok(())
    }

    /// Creates and saves a saved file from the single examination game mode to project dir
    pub fn save(&self) -> Result<String> {
        let file_name = format!(
            "{}_single_exam_save_file.toml",
            Local::now().format("%d-%m-%Y_%H:%M")
        );
        let file_path: &Path = Path::new(&file_name);
        self.save_to_path(file_path)
            .expect("failed to fully save file");
        Ok(file_name)
    }
}

#[derive(Clone)]
pub struct QuizList(Vec<Quiz>);

impl QuizList {
    /// creates a quizes struct for loading all quizes to display to user
    pub fn load_stored_quizes() -> Result<QuizList> {
        let mut cached_quizes: Vec<Quiz> = Vec::new();
        let paths = fs::read_dir("src/quizes/").unwrap();

        for path in paths {
            cached_quizes.push(Quiz::load_quiz_from_toml(&path.unwrap().path())?);
        }
        let quizes = QuizList(cached_quizes);
        Ok(quizes)
    }

    /// Search quizes struct for user input quiz to prepare for test taking.
    pub fn ready_quiz(self, input_quiz_name: String) -> Option<Quiz> {
        let mut quiz: Option<Quiz> = None;

        for single_quiz in self.0 {
            if input_quiz_name == single_quiz.quiz_name.trim().to_lowercase() {
                quiz = Some(single_quiz.clone());
            }
        }
        quiz
    }
}

impl fmt::Display for QuizList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for quiz in &self.0 {
            write!(f, "{}", quiz.quiz_name)?
        }
        Ok(())
    }
}

/// Main Structure for single examination, holds collection of questions for user to answer.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: Vec<Question>,
}

impl Quiz {
    pub fn load_quiz_from_toml(path: &Path) -> Result<Quiz> {
        let toml_str = fs::read_to_string(path)?;
        let quiz: Quiz = toml::from_str(&toml_str)?;
        Ok(quiz)
    }

    pub fn begin_quiz(self, answered_questions: Option<HashMap<String, bool>>) -> Option<u32> {
        let mut score = 0;
        let mut loaded_saved_quiz = false;
        let mut save_and_quit_prompt = false;
        let mut answered_questions_record: HashMap<String, bool> = match answered_questions {
            None => HashMap::new(),
            Some(answered_questions) => {
                loaded_saved_quiz = true;
                answered_questions
            }
        };

        // Cycle through questions
        for question in &self.questions {
            if loaded_saved_quiz {
                // TODO: obviously very ineffiecent, as if you loaded, this is growing throughout the test
                // ideally you would strip those and have the relevent information seperate?
                // Maybe we can solve with quiz metadata? Or clone og, strip down one for test taking and use
                // the og for results?
                match SavedQuiz::check_answered_question(
                    &question.question,
                    &answered_questions_record,
                ) {
                    None => (),
                    Some(is_answer_correct) => {
                        if is_answer_correct {
                            score += 1;
                        }
                        continue; // skip question since it was answered
                    }
                }
            }
            match question.ask() {
                Some(true) => {
                    score += 1;
                    answered_questions_record.insert(question.question.clone(), true);
                }
                Some(false) => {
                    answered_questions_record.insert(question.question.clone(), false);
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
            let saved_quiz = SavedQuiz {
                quiz_in_progress: self,
                answered_questions: answered_questions_record,
            };
            match saved_quiz.save() {
                Ok(file_name) => {
                    println!("Progess saved at {file_name}.");
                    return None;
                }
                Err(e) => {
                    println!("Something went wrong, save file cannot be generated: {e}");
                    return None; // TODO: Loss of save file is pretty bad, find a way to cycle back into game or prompt user for choice.
                }
            };
        }
        Some(score)
    }

    /// creates a grade struct and prints the outcome of a quiz given a score and total quiz questions
    /// it is largly standalone since score is not saved in the struct.
    pub fn show_result(score: u32, total_quiz_question: &u32) {
        tools::clear_terminal();
        let user_grade_percentage: u8 = (score * 100 / total_quiz_question) as u8;
        let user_grade = Grade::from(user_grade_percentage);
        println!(
            "You got {} / {} correct. --- {}%",
            score, total_quiz_question, user_grade_percentage
        );
        println!("{user_grade}");
        user_grade.print_random_grade_message();
    }

    /// returns number of questions for a given quiz
    pub fn get_quiz_length(&self) -> u32 {
        self.questions.len() as u32
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_answer: i8,
}

impl Question {
    fn ask(&self) -> Option<bool> {
        tools::clear_terminal();
        let mut rng = thread_rng();

        println!("{}", self.question);
        // original answer vector will act as answer key
        let mut shuffled_answers = self.answers.clone();
        shuffled_answers.shuffle(&mut rng);

        for suffled_answer in shuffled_answers.iter().enumerate() {
            println!("[{}] {}", suffled_answer.0 + 1, suffled_answer.1);
        }
        println!("Enter the number next to the answer you beleive is correct.");
        println!("Type 'save and quit' if you would like to do so.");
        loop {
            let user_input = tools::read_input();

            if user_input == "save and quit" {
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
            if !(1..5).contains(&user_answer) {
                println!("Invaild input, please enter one of the available answers.");
                continue;
            }
            println!();
            if shuffled_answers[user_answer - 1] == self.answers[self.correct_answer as usize - 1] {
                return Some(true);
            } else {
                return Some(false);
            }
        }
    }
}

#[derive(PartialEq, Debug)]

enum LetterGrade {
    A,
    B,
    C,
    D,
    F,
}
impl fmt::Display for LetterGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LetterGrade::A => write!(f, "A"),
            LetterGrade::B => write!(f, "B"),
            LetterGrade::C => write!(f, "C"),
            LetterGrade::D => write!(f, "D"),
            LetterGrade::F => write!(f, "F"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum LetterGradeModifier {
    Plus,
    Mid,
    Minus,
}

struct Grade {
    grade: LetterGrade,
    modifer: LetterGradeModifier,
}

impl Grade {
    const A_MESSAGES: &'static [&'static str; 5] = &[
        "Fantastic work!",
        "Oh shit!",
        "Jesus!",
        "Fucking Hell!",
        "I'm not worthy of your presence!",
    ];
    const B_MESSAGES: &'static [&'static str; 5] = &[
        "Nice.",
        "Could've been better.",
        "Alright, good job!",
        "Close enough I suppose.",
        "Nice work keeping above C level.",
    ];
    const C_MESSAGES: &'static [&'static str] = &[
        "Acceptable.",
        "Ok. Sure.",
        "Nothing special.",
        "Cs get degrees.",
    ];
    const D_MESSAGES: &'static [&'static str; 5] = &[
        "Cutting it close eh?",
        "Bah, you'll get em next time.",
        "Hey, that's passing right?",
        "Do better.",
        "You got this. Never surrender. Give it another try.",
    ];
    const F_MESSAGES: &'static [&'static str; 7] = &[
        "Damn, you fucking suck.",
        "Jesus man. Really?",
        "Were you even trying?",
        "Looks like all those brain cells really are gone.",
        "Sheeesh, nice work bro.",
        "Fucking dumb ass.",
        "Bruh",
    ];
}

impl Grade {
    /// prints a random message from the stored grade messages.
    pub fn print_random_grade_message(self) {
        let mut rng = thread_rng();
        let messages = match self.grade {
            LetterGrade::A => Grade::A_MESSAGES,
            LetterGrade::B => Grade::B_MESSAGES,
            LetterGrade::C => Grade::C_MESSAGES,
            LetterGrade::D => Grade::D_MESSAGES,
            LetterGrade::F => Grade::F_MESSAGES,
        };

        for message in messages.choose_multiple(&mut rng, 1) {
            println!("{message}");
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.modifer == LetterGradeModifier::Plus {
            write!(f, "Grade: {}+", self.grade)
        } else if self.modifer == LetterGradeModifier::Mid {
            write!(f, "Grade: {}", self.grade)
        } else {
            write!(f, "Grade: {}-", self.grade)
        }
    }
}

impl From<u8> for Grade {
    fn from(score: u8) -> Self {
        match score {
            97..=255 => Grade {
                // Should never be over 100
                grade: LetterGrade::A,
                modifer: LetterGradeModifier::Plus,
            },
            94..=96 => Grade {
                grade: LetterGrade::A,
                modifer: LetterGradeModifier::Mid,
            },
            90..=93 => Grade {
                grade: LetterGrade::A,
                modifer: LetterGradeModifier::Minus,
            },
            87..=89 => Grade {
                grade: LetterGrade::B,
                modifer: LetterGradeModifier::Plus,
            },
            84..=86 => Grade {
                grade: LetterGrade::B,
                modifer: LetterGradeModifier::Mid,
            },
            80..=83 => Grade {
                grade: LetterGrade::B,
                modifer: LetterGradeModifier::Minus,
            },
            77..=79 => Grade {
                grade: LetterGrade::C,
                modifer: LetterGradeModifier::Plus,
            },
            74..=76 => Grade {
                grade: LetterGrade::C,
                modifer: LetterGradeModifier::Mid,
            },
            70..=73 => Grade {
                grade: LetterGrade::C,
                modifer: LetterGradeModifier::Minus,
            },
            67..=69 => Grade {
                grade: LetterGrade::D,
                modifer: LetterGradeModifier::Plus,
            },
            64..=66 => Grade {
                grade: LetterGrade::D,
                modifer: LetterGradeModifier::Mid,
            },
            60..=63 => Grade {
                grade: LetterGrade::D,
                modifer: LetterGradeModifier::Minus,
            },
            50..=59 => Grade {
                grade: LetterGrade::F,
                modifer: LetterGradeModifier::Plus,
            },
            0..=49 => Grade {
                grade: LetterGrade::F,
                modifer: LetterGradeModifier::Minus,
            },
        }
    }
}

/// testing private functions
#[cfg(test)]
mod tests {
    use crate::riddler::*;
    use tempfile::tempdir;

    fn create_test_question() -> Question {
        Question {
            question: "Test Question".to_string(),
            answers: vec![
                "Test Answer1".to_string(),
                "Test Answer2".to_string(),
                "Test Answer3".to_string(),
                "Test Answer4".to_string(),
            ],
            correct_answer: 1,
        }
    }
    fn create_test_quiz() -> Quiz {
        Quiz {
            quiz_name: "Test Quiz".to_string(),
            questions: vec![create_test_question()],
        }
    }
    fn create_test_quizes() -> QuizList {
        QuizList(vec![create_test_quiz()])
    }
    fn create_test_saved_quiz() -> SavedQuiz {
        SavedQuiz {
            quiz_in_progress: create_test_quiz(),
            answered_questions: HashMap::new(),
        }
    }

    #[test]
    fn test_read_quiz() {
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
            Quiz::show_result(score, &100)
        }
        for total_questions in 1..100 {
            Quiz::show_result(1, &total_questions)
        }
    }

    /// Testing outside operable range
    #[test]
    fn test_show_result_1() {
        Quiz::show_result(11, &10)
    }
    #[test]
    #[should_panic]
    fn test_show_result_2() {
        Quiz::show_result(u32::MAX, &10)
    }

    #[test]
    fn serde_integreation_testing() {
        // feel like is a mess. for now know that the file path is "/tmp/.tmpqo6Lfk/Test_quiz.toml"
        let dir = tempdir().expect("Failed to create tmp dir");
        let file_path = dir
            .path()
            .join("Test_quiz.toml")
            .into_os_string()
            .into_string()
            .unwrap();

        // testing saving file
        let file_path_to_save = Path::new(&file_path);

        let saved_quiz = create_test_saved_quiz();
        match saved_quiz.save_to_path(&file_path_to_save) {
            Ok(_) => (),
            Err(_) => panic!("Failed to save file"),
        }

        // testing load saved file
        let test_saved_quiz = create_test_saved_quiz();
        // let file_path: &Path = Path::new(&"Test_quiz.toml");

        assert_eq!(
            test_saved_quiz.quiz_in_progress.quiz_name,
            SavedQuiz::load(Path::new(&file_path))
                .unwrap()
                .quiz_in_progress
                .quiz_name,
            "Failed to load saved quiz state"
        );
    }

    #[test]
    fn test_load_quiz_from_toml() {
        let dir = tempdir().expect("Failed to create tmp dir");
        let file_path = dir
            .path()
            .join("Test_quiz.toml")
            .into_os_string()
            .into_string()
            .unwrap();

        let saved_quiz = format!(
            r#"
quiz_name = "Test Quiz"
[[questions]]
question = "Test Question"
answers = [
    "Test Answer1",
    "Test Answer2",
    "Test Answer3",
    "Test Answer4"
    ]
correct_answer = 1
"#
        );
        // let saved_file: String = toml::to_string(&saved_quiz).expect("Failed to serialize toml file");
        let mut file = File::create(file_path.clone()).unwrap();
        match file.write(saved_quiz.as_bytes()) {
            Ok(_) => println!("file writen successfully"),
            Err(_) => panic!(),
        }

        // start of test
        let file_path: &Path = Path::new(&file_path);
        println!("{}", file_path.display());
        let quiz = match Quiz::load_quiz_from_toml(file_path) {
            Ok(quiz) => quiz,
            Err(e) => {
                println!("Something went wrong!: {e}");
                panic!()
            }
        };
        assert_eq!("Test Quiz", quiz.quiz_name);
    }

    #[test]
    fn test_check_answered_questions() {
        let question_number = "question1".to_string();
        let mut arr_of_answered_questions1 = HashMap::new();
        let mut arr_of_answered_questions2 = HashMap::new();
        arr_of_answered_questions1.insert("question1".to_string(), false);
        arr_of_answered_questions2.insert("question2".to_string(), false);

        assert_eq!(
            Some(false),
            SavedQuiz::check_answered_question(&question_number, &arr_of_answered_questions1)
        );
        assert_eq!(
            None,
            SavedQuiz::check_answered_question(&question_number, &arr_of_answered_questions2)
        );
    }
    #[test]
    fn test_print_random_grade_message() {
        let a = Grade::from(100);
        assert_eq!(a.grade, LetterGrade::A);
        assert_eq!(a.modifer, LetterGradeModifier::Plus);

        let b = Grade::from(87);
        assert_eq!(b.grade, LetterGrade::B);
        assert_eq!(b.modifer, LetterGradeModifier::Plus);

        let c = Grade::from(75);
        assert_eq!(c.grade, LetterGrade::C);
        assert_eq!(c.modifer, LetterGradeModifier::Mid);

        let d = Grade::from(63);
        assert_eq!(d.grade, LetterGrade::D);
        assert_eq!(d.modifer, LetterGradeModifier::Minus);

        let f = Grade::from(0);
        assert_eq!(f.grade, LetterGrade::F);
        assert_eq!(f.modifer, LetterGradeModifier::Minus);

        a.print_random_grade_message();
        b.print_random_grade_message();
        c.print_random_grade_message();
        d.print_random_grade_message();
        f.print_random_grade_message();
    }
}
