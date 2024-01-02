extern crate yaml_rust;
use std::collections::HashMap;

pub struct QuizQuestion { // not in use
    pub parent_quiz: String,
    pub question: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: Option<String>,
    pub answer4: Option<String>,
    pub correct_answer: i8 // should be whatever corresponds to the correct answer. 
    // maybe change later if we wanna add more types of questions. but for now. only true false or multi 4 op. 
}


#[derive(Debug)]
pub struct QuizQuestionv2 {
    pub question_and_answers: Vec<HashMap<String, String>>,
    // maybe change later if we wanna add more types of questions. but for now. only true false or multi 4 op. 
    // wonder if there is any point in making this a struct. Feel like there isnt if you gonna have everything in
    // this one thing.?
    // This could all just be in a quiz, change the funciton to generate_test_quiz_questions
    // then have a couple of other methods on quiz init. 
    // but lets think about that later. now we wanna get this working. 
}

#[derive(Debug)]
pub struct Quiz {
    pub quiz_name: String,
    pub quiz_questions: QuizQuestionv2,
    pub quiz_metadata: Vec<Option<String>> // points, statistics, notes? 
    // what else do we need in here though? why make this a struct? 
}

impl QuizQuestionv2{
    
    /// returns the questions of a quiz in a struct. 
    /// note by the nature of hash these will not be in order 
    /// everything within a quesiton that is, not the order of the questions
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
    /// quiz for early testing
    pub fn create_dev_quiz() -> Self {
        let test_quiz = Quiz{
            quiz_name: String::from("test quiz for testing!"),
            quiz_questions: QuizQuestionv2::create_test_quiz_questions(),
            quiz_metadata: vec![None, None]
        };
        test_quiz
    }
}