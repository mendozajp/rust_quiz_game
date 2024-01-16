use std::collections::HashMap;
use std::{fs, path::Path};
extern crate yaml_rust;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]

pub struct Quiz {
    pub quiz_name: String,
    pub questions: HashMap<String, Question>,
}

#[derive(Debug, Deserialize)]
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

    return quiz;
}
