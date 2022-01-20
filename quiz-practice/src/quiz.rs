use crate::input;
use std::fs;
use std::io::Read;

pub fn input_quiz(quiz_tsv_filename: &str) -> Vec<(String, String)> {
    let mut quiz_file = fs::File::open(quiz_tsv_filename).expect("File Open");
    let mut buf = String::new();
    quiz_file.read_to_string(&mut buf).expect("Reading File");

    let q_and_a: Vec<(String, String)> = buf
        .lines()
        .map(|line| {
            let mut q_a = line.split(',').map(|s| s.to_string());
            let question = q_a.next().expect("No question found");
            let answer = q_a.next().expect("No answer found");
            (question, answer)
        })
        .collect();

    println!("{:#?}", q_and_a);

    q_and_a
}

pub fn test_quiz(q_and_a: &Vec<(String, String)>) -> (usize, usize) {
    let correct_num = q_and_a
        .into_iter()
        .map(|question| test_question(question))
        .filter(|val| *val)
        .count();
    (correct_num, q_and_a.len())
}

pub fn test_question(q_and_a: &(String, String)) -> bool {
    println!("Question: {}", q_and_a.0);
    let ans = input("Answer: ").expect("Input Answer failed");
    ans == q_and_a.1
}
