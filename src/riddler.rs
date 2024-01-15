use std::{collections::HashMap, path::Path, fs};
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, yaml::Hash, Yaml};

/// not in use
// pub struct QuizQuestion { 
//     pub parent_quiz: String,
//     pub question: String,
//     pub answer1: String,
//     pub answer2: String,
//     pub answer3: Option<String>,
//     pub answer4: Option<String>,
//     pub correct_answer: i8
// }


#[derive(Debug)]
pub struct QuizQuestionv2 {
    pub question_and_answers: Vec<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Quiz {
    pub quiz_name: String,
    pub quiz_questions: QuizQuestionv2,
    pub quiz_metadata: Vec<Option<String>> // points, statistics, notes? 
    // what else do we need in here though? why make this a struct? 
}

impl QuizQuestionv2 {
    
    /// returns the questions of a quiz in a struct. 
    /// note by the nature of hash these will not be in order 
    pub fn create_test_quiz_questions() -> Self{
        let question1 = HashMap::from([
            (String::from("Question Name"), String::from("What is Jordy's name?")),
            (String::from("A1"), String::from("Jordy")),
            (String::from("A2"), String::from("Simon")),
            (String::from("A3"), String::from("Jeffry")),
            (String::from("A4"), String::from("Richard")),
            (String::from("Answer"), String::from("A1")),
        ]);

        let question2 = HashMap::from([
            (String::from("Question Name"), String::from("What is Jordy's favorite color?")),
            (String::from("A1"), String::from("Blue")),
            (String::from("A2"), String::from("Red")),
            (String::from("A3"), String::from("Orange")),
            (String::from("A4"), String::from("Black")),
            (String::from("Answer"), String::from("A4")),
        ]);

        let question3 = HashMap::from([
            (String::from("Question Name"), String::from("What is Jordy's favorite drink?")),
            (String::from("A1"), String::from("Tea")),
            (String::from("A2"), String::from("Coffee")),
            (String::from("A3"), String::from("Soda")),
            (String::from("A4"), String::from("Beer")),
            (String::from("Answer"), String::from("A2")),
        ]);

        Self {
            question_and_answers: vec![question1, question2, question3]
        }
    }
}


impl Quiz{

    /// create quiz object for user
    fn create_quiz(quiz_name: String,
         quiz_questions:QuizQuestionv2,
          quiz_metadata:Vec<Option<String>>)
           -> Self {
        Quiz{
            quiz_name,
            quiz_questions,
            quiz_metadata,
        }
    }
    /// quiz for testing
    pub fn create_dev_quiz() -> Self {
        let test_quiz = Quiz{
            quiz_name: String::from("test quiz for testing!"),
            quiz_questions: QuizQuestionv2::create_test_quiz_questions(),
            quiz_metadata: vec![None, None]
        };
        test_quiz
    }
}


/// loads a quiz from a yaml file. Yaml structure must be consistent across all quizes. 
/// see src/quizes/test_quiz.yaml for structure. 
pub fn load_quiz_from_yaml(path: &Path) -> Quiz {
    let mut quizes: Vec<Quiz> = Vec::new();
    let file_contents = fs::read_to_string(path)
        .expect("Should have been able to read the file. Is this the correct path?");

    let quiz_file = YamlLoader::load_from_str(&file_contents).unwrap();
    let mut quiz_name = String::new();
    let mut questions_and_answers: Vec<HashMap<String, String>> = Vec::new();
    let mut real_question_answer = QuizQuestionv2::create_test_quiz_questions();
    let quiz_metadata: Vec<Option<String>> = vec![None, None];


    for questions in quiz_file[0].as_hash().unwrap() { // will only get one file/quiz
        quiz_name = questions.0.as_str().unwrap().to_string();

        for question in questions.1.as_hash().unwrap() {
            let question_name = question.0.clone().into_string();
            let mut available_answers: HashMap<String, String> = HashMap::new();

            let lines = question.1.clone().into_hash().unwrap();
            let mut counter = 0;
            for k in lines{
                // bruh
                counter += 1;
                let mut key = Some(String::from("")); 
                let mut answer = Some(String::from("")); 
                let mut emergency = Some(90);
                let mut key_flag = false;
                let mut value_flag = false;
                let mut failsafe = String::new();

                // if value being read is int, needs different conversion
                match k.0.clone().into_string() {
                    Some(value) => {key = Some(value)}
                    None => {
                        key_flag = true;
                        emergency = k.0.as_i64();
                    }
                }

                // if value being read is int, needs different conversion
                match k.1.clone().into_string() {
                    Some(value) => {answer = Some(value)}
                    None => {
                        value_flag = true;
                        emergency = k.1.as_i64();
                    }
                }

                // handle if either key or value is int
                if key_flag || value_flag {
                    failsafe = emergency.unwrap().to_string();
                    if key_flag{
                        available_answers.insert(failsafe, answer.unwrap());
                    }
                    else if value_flag{
                        available_answers.insert(key.unwrap(), failsafe);
                    }
                }
                 else {
                    available_answers.insert(key.unwrap(), answer.unwrap());
                }
            }
            available_answers.insert(String::from("Question Name"), question_name.unwrap());
            questions_and_answers.push(available_answers);
        }
        real_question_answer = QuizQuestionv2{question_and_answers: questions_and_answers.clone()};
    }
    Quiz::create_quiz(quiz_name, real_question_answer, quiz_metadata)
}