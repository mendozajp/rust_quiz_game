mod riddler;

fn main() {
    let test_quiz = riddler::Quiz{
        quiz_name: String::from("test quiz for testing!"),
        quiz_questions: riddler::QuizQuestion_v2::create_test_quiz_questions(),
        quiz_metadata: vec![None, None]
    };

    println!("{:#?}", test_quiz);
}
