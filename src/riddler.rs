use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fmt, path::Path};
use std::{fs, fs::File};

use include_dir::{include_dir, Dir};

// rebuild if you add any new quizzes
static PROJECT_DIR: Dir = include_dir!("src/quizzes/");

use crate::tools;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ReadyQuiz {
    pub quiz_name: String,
    pub questions: Vec<Question>,
}

impl ReadyQuiz {
    fn ready_quiz_to_quiz(self) -> Quiz {
        Quiz {
            quiz_name: self.quiz_name,
            questions: self.questions,
            user_answers: Vec::<(Question, String)>::new(),
            score: 0,
        }
    }

    pub fn load_included_quizes() -> Result<Vec<Quiz>> {
        let mut cached_quizes: Vec<Quiz> = Vec::new();
        for entry in PROJECT_DIR.find("*.toml")? {
            let quiz_contents = PROJECT_DIR
                .get_file(entry.path())
                .expect("could not find file")
                .contents_utf8()
                .expect("could not retreive contents");
            let deserial_attempt: ReadyQuiz = toml::from_str(&quiz_contents)?;
            // dbg!(&deserial_attempt);
            cached_quizes.push(deserial_attempt.ready_quiz_to_quiz());
        }
        return Ok(cached_quizes);
    }
}

#[derive(Clone)]
pub struct QuizList(pub Vec<Quiz>);

impl QuizList {
    /// creates a quizes struct for loading all quizes to display to user
    pub fn load_stored_quizes() -> Result<QuizList> {
        let cached_quizes = ReadyQuiz::load_included_quizes()?;
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
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: Vec<Question>,
    pub user_answers: Vec<(Question, String)>,
    pub score: u32,
}

impl Quiz {
    /// if question name in answered questions, the score should reflect it
    fn check_answered_question(&self, current_question: &Question) -> bool {
        for answered_question in &self.user_answers {
            if &answered_question.0 == current_question {
                println!("this question has been answered - ignoring");
                return true;
            } else {
                continue;
            }
        }
        false
    }

    /// Load a saved quiz progress into memory
    pub fn load(path: &Path) -> Result<Quiz> {
        let toml_str = fs::read_to_string(path)?;
        let quiz: Quiz = toml::from_str(&toml_str)?;
        Ok(quiz)
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

    pub fn begin_quiz(mut self) -> Option<Quiz> {
        let mut save_and_quit_prompt = false;
        let mut loaded_saved_quiz = false;
        let mut new_user_answers: Vec<(Question, String)> = Vec::new();
        if self.user_answers.len() > 0 {
            loaded_saved_quiz = true;
        }

        // Cycle through questions
        for question in &self.questions {
            if loaded_saved_quiz {
                if self.check_answered_question(&question) {
                    continue; // skip question since it was answered
                }
            }

            match question.ask() {
                // TODO: IM NOT SURE YOU CAN MATCH TUPLES LIKE THIS
                Some((user_answer, true)) => {
                    self.score += 1;
                    new_user_answers.push((question.clone(), user_answer));
                }
                Some((user_answer, false)) => {
                    new_user_answers.push((question.clone(), user_answer));
                    continue;
                }
                None => {
                    // only 'save and quit' input by user
                    save_and_quit_prompt = true;
                    break;
                }
            }
        }
        // combine new user answers with user answers for reporting and saving
        for answer in new_user_answers {
            self.user_answers.push(answer);
        }

        if save_and_quit_prompt {
            let saved_quiz = self;
            match saved_quiz.save() {
                Ok(file_name) => {
                    println!("Progess saved at {file_name}.");
                    return None;
                }
                Err(e) => {
                    println!("Something went wrong, save file cannot be generated: {e}");
                    return None;
                }
            };
        }
        Some(self)
    }

    /// creates a grade struct and prints the outcome of a quiz given a score and total quiz questions
    /// it is largly standalone since score is not saved in the struct.
    pub fn show_result(self) {
        tools::clear_terminal();
        let user_grade_percentage: u8 = (self.score * 100 / &self.get_quiz_length()) as u8;
        let user_grade = Grade::from(user_grade_percentage);
        println!(
            "You got {} / {} correct. --- {}%",
            self.score,
            self.questions.len(),
            user_grade_percentage
        );
        println!("{user_grade}");
        user_grade.print_random_grade_message();

        println!("type 'answers' if you would like to see what you got right and wrong. Otherwise just hit enter.");
        loop {
            let prompt = tools::read_input();
            if prompt == "" {
                break;
            } else if prompt == "answers" {
                self.display_user_answers();
                break;
            } else {
                println!("Either type 'answer' to view answers or just press enter to return to main menu...");
            }
        }
    }

    /// returns number of questions for a given quiz
    pub fn get_quiz_length(&self) -> u32 {
        self.questions.len() as u32
    }

    pub fn display_user_answers(&self) {
        for report in &self.user_answers {
            println!("Question: {}", report.0.question);
            if report.0.answers[(report.0.correct_answer - 1) as usize] == report.1 {
                println!("{} {}", "Your answer:".green(), report.1.green());
            } else {
                println!("{} {}", "Your answer:".red(), report.1.red());
                println!(
                    "{} {}",
                    "Correct Answer:".green(),
                    report.0.answers[(report.0.correct_answer - 1) as usize].green()
                );
            }
            println!();
        }
        let user_grade_percentage: u8 = (self.score * 100 / &self.get_quiz_length()) as u8;

        println!(
            "You got {} / {} correct. --- {}%",
            self.score,
            self.questions.len(),
            user_grade_percentage
        );
        println!("Press enter to return to main menu.");
        tools::read_input();
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_answer: i8,
}

impl Question {
    fn ask(&self) -> Option<(String, bool)> {
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
                return Some((shuffled_answers[user_answer - 1].clone(), true));
            } else {
                return Some((shuffled_answers[user_answer - 1].clone(), false));
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
