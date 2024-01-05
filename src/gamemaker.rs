// this file should house all the logic for actually running the quizes, so higher level then riddler. 
// to be a library though, wont be necessary for first iteration
// mod riddler;
use std::{io, collections::{hash_map, HashMap}};
#[path = "riddler.rs"]
mod riddler;
use std::io::BufRead;

pub fn start_up_screen() {
    println!("Welcome To Quiz Show!\n");
    println!("Currently the game is in its infancy so we only have a dev testing quiz.");
    println!("Give it a shot, see how it works.");
    println!("The quiz will start when you hit enter.");

    let stdin = io::stdin(); // TODO: upon running this does not do what we think it does. 
    stdin.lock().lines().next();
    let results = start_quiz(ready_quiz("test_dev"));
    report_outcome(results);
}

/// Arguement should probably be the name of the quiz you would want
/// returned by start up screen or some other screen selector. 
/// We might make this cache quizes later but im not sure yet. 
fn ready_quiz(test_name: &str) -> riddler::Quiz {
    if test_name == "dev_test"{
        riddler::Quiz::create_dev_quiz()
    }
    else{
        panic!("We don't have real quizes yet!!")
    }
}


fn start_quiz(quiz: riddler::Quiz) -> Vec<bool>{
    // you should do a bash reset here, look into it later
    // looks like we dont need this but im setting up for initing the env later. 
    cycle_through_questions(quiz.quiz_questions)
}

/// only thing this should say now is how many you got right out of how many you got wrong. 
fn report_outcome(results: Vec<bool>) {
    let total_number_of_questions = results.len();
    let number_of_correct_answers = results.iter().filter(|&result| *result == true).count();

    println!("You got {} out of {} correct! Thank you for playing!", 
        number_of_correct_answers, total_number_of_questions);
}

/// display current question and answers, provide numbers for user to enter 
fn display_question(question: HashMap<String,String>) -> bool{
    let mut list_of_answers: Vec<(usize,&String)> = Vec::new();
    let question_name = question.get("Question Name").unwrap(); // TODO: Added unwrap here. is that the only way? confirm
    println!("{question_name}"); // TODO: not sure how to address this problem yet, in right way that is
    // no display implemenation that is.
    println!("\n");

    for (key, value) in &question{
        if key == "Question Name" || key == "Answer" { continue; };
        list_of_answers.push((list_of_answers.len() + 1, value));
    }

    for answer in &list_of_answers{
        println!("[{}] {}",answer.0, answer.1);
    };

// TODO: 
    // objective here is to get the user input, confirm its good to convert to a number
    // then find the numbers corresponding answer, then find that answers corresponding key in 
    // the question object since that key is the same format as the answer, then check if the user
    // guess is the correct answer and return a bool on what it is.

    loop{
        let mut user_guess: String = String::new();
        println!("Please enter your guess by typing its corresponding number.");
        io::stdin()
            .read_line(&mut user_guess)
            .expect("");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invaild input, please enter your guess by typing its corresponding number");
                continue;
            }
        };

        // handle not available option
        if user_guess > list_of_answers.len().try_into().unwrap() || user_guess <= 0 { // TODO: confirm need for try_into and unwrap
            println!("input not available, please enter your guess by typing its corresponding number");
            continue;
        }
        
        
        // handle looking for key of value input 
        list_of_answers.iter()
        .find_map(|(key, &ref val)| if &list_of_answers[1].0.to_string() == val { Some(key) } else { None });
    // TODO: Still need to do something with this. consider removing all together since im starting to lose the thread. 

        // handle return bool
    }
}

fn cycle_through_questions(questions: riddler::QuizQuestionv2) -> Vec<bool>{
    // not sure if i want to do this yet
    let mut results: Vec<bool> = Vec::new();

    for question in questions.question_and_answers{
        results.push(display_question(question));
    }
    results
}