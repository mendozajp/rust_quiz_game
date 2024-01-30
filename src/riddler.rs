use chrono::Local;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::{fs, fs::File, path::Path};
use toml;

#[path = "tools.rs"]
mod tools;

/// Load a toml quiz file into memory
pub fn load_single_exam_save_file(path: &Path) -> SavedQuiz {
    let toml_str = fs::read_to_string(path).expect("Failed to read toml file");
    let saved_quiz: SavedQuiz = toml::from_str(&toml_str).expect("Failed to deserialize toml file");

    return saved_quiz;
}

pub fn create_and_save_single_exam_save_file(
    quiz_to_save: Quiz,
    answered_questions: Vec<bool>,
) -> std::io::Result<()> {
    let saved_quiz = SavedQuiz {
        ordered_quiz: quiz_to_save,
        answered_questions: answered_questions,
    };
    let saved_file: String = toml::to_string(&saved_quiz).expect("Failed to serialize toml file"); // we throw every result up, can we do the same with this? --- This will shape other uses of .expect depending on what we find. maybe look at some of H. example code?
    println!("DEBUG: {saved_file}");
    let file_name = format!(
        "{}_single_exam_save_file.toml",
        Local::now().format("%d-%m-%Y_%H:%M")
    );

    let mut file = File::create(file_name)?;
    file.write(saved_file.as_bytes())?;
    Ok(())

    // write to file
    // exit - maybe throw in a handle user action but with a param forcing a specific game state?
}

#[derive(Serialize, Deserialize)]
pub struct SavedQuiz {
    ordered_quiz: Quiz,
    answered_questions: Vec<bool>,
}

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

    pub fn setup_single_examination() -> Quizes {
        Quizes {
            available_quizes: Quizes::load_stored_quizes(),
        }
    }

    pub fn display_quiz_names(&self) {
        for quiz in &self.available_quizes {
            println!("{}", quiz.quiz_name);
        }
    }

    pub fn ready_quiz(self, input_quiz_name: String) -> Option<Quiz> {
        let mut quiz: Option<Quiz> = None;

        for single_quiz in self.available_quizes {
            if input_quiz_name == single_quiz.quiz_name.trim().to_lowercase() {
                quiz = Some(single_quiz.clone());
                println!(); // spacing
            }
        }

        return quiz;
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: HashMap<String, Question>,
}

impl Quiz {
    pub fn take_quiz(self) -> i32 {
        let mut score: i32 = 0;
        let answered_questions_record = Vec::new();

        // Cycle through questions
        for question in self.questions {
            match question.1.ask_question() {
                Some(true) => {
                    score += 1;
                }
                Some(false) => continue,
                None => {
                    match create_and_save_single_exam_save_file(
                        self.clone(),
                        answered_questions_record,
                    ) {
                        Err(_) => {
                            println!("Something went wrong with saving state. Abandoning.");
                            return -2;
                        }
                        Ok(_) => {
                            println!("Save State created. Quiting game...");
                            return -1;
                        }
                    }
                }
            }
        }
        score
    }
    pub fn show_result(score: i32, total_questions: i32) {
        println!(); // spacing
        let grade_number = score * 100 / total_questions;
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
                _ => {}
            }
        }

        println!(
            "You scored {score} out of {} -- {grade_number}%",
            total_questions
        );
        let grade: char;
        match grade_number {
            97..=100 => {
                grade = 'A';
                println!("Grade: {grade}+");
                _print_random_grade_message(grade);
            }
            94..=96 => {
                grade = 'A';
                println!("Grade: {grade}");
                _print_random_grade_message(grade);
            }
            90..=93 => {
                grade = 'A';
                println!("Grade: {grade}-");
                _print_random_grade_message(grade);
            }
            87..=89 => {
                grade = 'B';
                println!("Grade: {grade}+");
                _print_random_grade_message(grade);
            }
            84..=86 => {
                grade = 'B';
                println!("Grade: {grade}");
                _print_random_grade_message(grade);
            }
            80..=83 => {
                grade = 'B';
                println!("Grade: {grade}-");
                _print_random_grade_message(grade);
            }
            77..=79 => {
                grade = 'C';
                println!("Grade: {grade}+");
                _print_random_grade_message(grade);
            }
            74..=76 => {
                grade = 'C';
                println!("Grade: {grade}");
                _print_random_grade_message(grade);
            }
            70..=73 => {
                grade = 'C';
                println!("Grade: {grade}-");
                _print_random_grade_message(grade);
            }
            67..=69 => {
                grade = 'D';
                println!("Grade: {grade}+");
                _print_random_grade_message(grade);
            }
            64..=66 => {
                grade = 'D';
                println!("Grade: {grade}");
                _print_random_grade_message(grade);
            }
            60..=63 => {
                grade = 'D';
                println!("Grade: {grade}-");
                _print_random_grade_message(grade);
            }
            50..=59 => {
                grade = 'F';
                println!("Grade: {grade}");
                _print_random_grade_message(grade);
            }
            0..=49 => {
                grade = 'F';
                println!("Grade: {grade}-");
                _print_random_grade_message(grade);
            }
            _ => unreachable!(),
        }
        println!() // spacing
    }
}

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
    fn ask_question(self) -> Option<bool> {
        // this should be referenced, but moves the question here. i think. though its in a loop so it being in a different scope pretty much does that already.
        println!("{}", self.question);
        let mut answers = HashMap::new();
        answers.insert(1, self.answer1);
        answers.insert(2, self.answer2);
        answers.insert(3, self.answer3);
        answers.insert(4, self.answer4);

        for answer in answers.iter().enumerate() {
            println!("[{}] {}", answer.1 .0, answer.1 .1);
        }
        println!("Enter the number next to the answer you beleive is correct.");
        loop {
            // TODO: watch for save and quit prompt
            let user_input = tools::read_input();

            if user_input == "save and quit" {
                return None;
            }
            let user_answer: i8 = match user_input.parse() {
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
            if user_answer == self.correct_answer {
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
