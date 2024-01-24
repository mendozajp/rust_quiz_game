use serde::Deserialize;
use std::clone;
use std::collections::HashMap;
use std::{fs, path::Path};
use toml;

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

    pub fn display_quiz_names(self) {
        for quiz in self.available_quizes {
            println!("{}", quiz.quiz_name);
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Quiz {
    pub quiz_name: String,
    pub questions: HashMap<String, Question>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub question: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub answer4: String,
    pub correct_answer: i8,
}

/// Load a toml quiz file into memory
/// TODO: Add readability for multiple quizes in one file
pub fn load_quiz_from_toml(path: &Path) -> Quiz {
    let toml_str = fs::read_to_string(path).expect("Failed to read toml file");
    let quiz: Quiz = toml::from_str(&toml_str).expect("Failed to deserialize toml file");
    // println!("{:?}", quiz);

    return quiz;
}
