// use std::fs;
// use std::path::Path;
// use std::{collections::HashMap, io};

// use self::riddler::Quiz;
// #[path = "riddler.rs"]
// mod riddler;

// /// displays prompts for user to begin playing. This is the entry state of the game.
// pub fn start_up_screen() {
//     println!("Welcome To Quiz Show!\n");
//     println!("Loading all available quizes...");
//     let (quizes, quiz_names) = load_stored_quizes();
//     println!("Quizes loaded.");
//     println!("Type the name of one of the following quizes to run that quiz:");
//     display_available_quizes(&quizes);
//     println!("The quiz will start when you hit enter. Type 'exit' to quit.");

//     let mut chosen_quiz_name = String::new();

//     loop {
//         // until they type a quiz name or exit
//         io::stdin()
//             .read_line(&mut chosen_quiz_name)
//             .expect("Failed to read line");

//         let quiz_to_do = String::from(chosen_quiz_name.trim());

//         if quiz_to_do == "exit" {
//             break;
//         }

//         if quiz_names.contains(&quiz_to_do) {
//             let results = start_quiz(ready_quiz(&quiz_to_do, quizes));
//             report_outcome(results); // TODO: MAKE RETURN A BOOL FOR LOOP if they want to do another quiz
//             break;
//         } else {
//             println!("That is not an available quiz, please type the name of one of one of the ");
//             println!("available quizes or type 'exit' to quit the game.");
//             io::stdin()
//                 .read_line(&mut chosen_quiz_name)
//                 .expect("Failed to read line");
//         }
//     }
// }

// /// reads all yaml files in src/quizes and loaded them into memory
// /// to be displayed in startup screen.
// pub fn load_stored_quizes() -> (Vec<riddler::Quiz>, Vec<String>) {
//     let mut cached_quizes: Vec<Quiz> = Vec::new();
//     let mut name_of_quizes: Vec<String> = Vec::new();
//     let paths = fs::read_dir("src/quizes/").unwrap();

//     for path in paths {
//         cached_quizes.push(riddler::load_quiz_from_yaml(&path.unwrap().path()));
//     }
//     for quiz in &cached_quizes {
//         name_of_quizes.push(quiz.quiz_name.clone());
//     }
//     (cached_quizes, name_of_quizes)
// }

// /// print all quizes for user.
// pub fn display_available_quizes(cached_quizes: &Vec<riddler::Quiz>) {
//     for quiz in cached_quizes {
//         println!("{}", quiz.quiz_name);
//     }
// }

// /// Arguement should probably be the name of the quiz you would want
// /// returned by start up screen or some other screen selector.
// /// We might make this cache quizes later but im not sure yet.
// fn ready_quiz(test_name: &str, quizes: Vec<Quiz>) -> riddler::Quiz {
//     if test_name == "dev_test" {
//         return riddler::Quiz::create_dev_quiz();
//     }
//     for quiz in quizes {
//         if quiz.quiz_name == test_name {
//             return quiz;
//         }
//     }
//     return riddler::Quiz::create_dev_quiz();
// }

// /// changes state to quiz, implemented to house future work.
// fn start_quiz(quiz: riddler::Quiz) -> Vec<bool> {
//     cycle_through_questions(quiz.quiz_questions)
// }

// /// displays outcome of current quiz to user.
// fn report_outcome(results: Vec<bool>) {
//     let total_number_of_questions = results.len();
//     let number_of_correct_answers = results.iter().filter(|&result| *result == true).count();

//     if number_of_correct_answers > total_number_of_questions / 2 {
//         println!("Sheeesh, good shit!");
//     } else {
//         println!("Damn! You fucking suck...");
//     }

//     println!(
//         "You got {} out of {} correct! Thank you for playing!",
//         number_of_correct_answers, total_number_of_questions
//     );
// }

// /// display current question and answers, provide numbers for user to enter and return true or
// /// false if user chose the correct answer.
// fn display_question(question: HashMap<String, String>) -> bool {
//     // check with getting both question number and question
//     let mut list_of_answers: Vec<(&String, &String)> = Vec::new();

//     // display question name
//     let question_name = question.get("Question Name").unwrap(); // TODO: Added unwrap here. is that the only way? confirm
//     println!("{question_name}");

//     for (key, value) in &question {
//         if key == "Question Name" || key == "Answer" {
//             continue;
//         };
//         list_of_answers.push((key, value));
//     }

//     for (index, answer) in list_of_answers.iter().enumerate() {
//         println!("[{}] {}", index, answer.1);
//     }

//     let correct_value = &question["Answer"];
//     let mut user_input: u32; // TODO: Confirm this the best type
//     loop {
//         let mut user_guess = String::new();
//         println!("Please enter your guess by typing its corresponding number.");
//         io::stdin().read_line(&mut user_guess).expect("");

//         user_input = match user_guess.trim().parse() {
//             // TODO: confirm method of populating user_input
//             Ok(num) => num,
//             Err(_) => {
//                 println!(
//                     "invaild input, please enter your guess by typing its corresponding number"
//                 );
//                 continue;
//             }
//         };

//         // handle not available option
//         if user_input + 1 > list_of_answers.len().try_into().unwrap() || user_input < 0 {
//             // TODO: confirm need for try_into and unwrap
//             // TODO: clean up conditionals
//             println!(
//                 "input not available, please enter your guess by typing its corresponding number"
//             );
//             continue;
//         }
//         break;
//     }

//     let user_input: usize = user_input.try_into().unwrap(); // TODO: doesnt feel clean at all. read for a better way on this last step.

//     if list_of_answers[user_input].0 == correct_value {
//         return true;
//     }
//     return false;
// }

// /// displays and saves all answers given by user for a given quiz.
// fn cycle_through_questions(questions: riddler::QuizQuestionv2) -> Vec<bool> {
//     let mut results: Vec<bool> = Vec::new();

//     for question in questions.question_and_answers {
//         results.push(display_question(question));
//     }
//     results
// }
