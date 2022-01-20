extern crate chrono;

mod quiz;

use chrono::prelude::*;
use std::io;

fn main() {
    // Input Name and Age
    let name = input("Name: ").expect("Input name failed");
    println!("> Hello, {}!", name);

    let age = input("Age: ")
        .expect("Input age failed")
        .parse::<i32>()
        .expect("Invalid Age");

    println!("> Your age is {}.", age);

    let current_year = Utc::now().year();

    if age > 100 {
        println!("You already turned 100 in {}!", 100 - age + current_year)
    } else {
        println!("You'll turn 100 in {}!", 100 - age + current_year);
    }

    // Input Odd or Even
    let num = input("Number: ")
        .expect("Input Number failed")
        .parse::<u16>()
        .expect("Invalid Number");

    let parity = match num % 2 {
        0 => "Even".to_string(),
        1 => "Odd".to_string(),
        _ => unreachable!("Parity failed"),
    };

    // Match
    println!("> {} is a {} number.", num, parity);

    // Ternary;
    println!(
        "> {} is a {} number.",
        num,
        if num % 2 == 0 { "Even" } else { "Odd" }
    );

    // quiz
    let q_and_a: Vec<(String, String)> = quiz::input_quiz("src/quiz.csv");
    // println!("{}", quiz::test_question(&q_and_a[0]))
    println!("Score: {:?}", quiz::test_quiz(&q_and_a));
}

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;
    print!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}
