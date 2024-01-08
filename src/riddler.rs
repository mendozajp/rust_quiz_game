use std::{collections::HashMap, path::Path, fs};
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, yaml::Hash};
// use core::any::type_name;
use std::any::type_name;

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


pub fn load_quiz_from_yaml(path: &Path) -> Quiz{
    //TODO: Confirm path exists and catch
    let file_contents = fs::read_to_string(path)
        .expect("Should have been able to read the file. Is this the correct path?");

    let quiz_file = YamlLoader::load_from_str(&file_contents).unwrap();

    // println!("{:#?}", type_of(&quiz_file));
    println!("{:#?}", type_of(&quiz_file[0][0]));
    println!("{:#?}", type_of(&quiz_file[0]));


    for questions in quiz_file[0].as_hash().unwrap() { // will only get one file/quiz
        let quiz_name = questions.0.as_str().unwrap().to_string();
        let questions_and_answers: Vec<HashMap<String, String>> = Vec::new();
        let quiz_metadata = vec![None, None];

        for question in questions.1.as_hash().unwrap() {
            // here you are going to be looping over all of the questions.
            // .1 is a hashmap with all the answers and the answer and .0 is the question name
            // if you can put .0 in .1s hash map with "question name" then we can just push it 
            // into questions and answers. 


            // TODO: FIND OUT HOW TO GET THAT HASH NOT YAML, LOOK INTO DUMP
            // THERE IS NO WAY THERE ISNT A WAY. IT WOULD BE DUMB IF THEIR WASNT.
            let question_name = question.0.as_str().unwrap().to_string();
            let available_answers = question.1.into_hash().unwrap();
            for (thing, thing1) in available_answers {
                thing = thing.into_string();
            }
            println!("{:?}", available_answers);
            available_answers.entry(String::from("Question Name")).or_insert(question_name);




            println!("{:?}", question);
    
        }

        Quiz::create_quiz(quiz_name, questions_and_answers, quiz_metadata);
        // println!("{:#?}", type_of(questions));
        // println!("*************************************************************************************");

    }



    println!("ignore return, its just for now...");
    Quiz::create_dev_quiz()
}

pub fn populate_master_quiz() {
    // read all files in quiz folder and populate master quiz.
    // not in scope for 1st Deliverable
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}