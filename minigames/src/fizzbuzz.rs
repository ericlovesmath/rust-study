pub fn print_up_to(num: usize) {
    for i in 1..num {
        // Why can't I just wrap this in println!()
        match (i % 5, i % 3) {
            (0, 0) => println!("Fizzbuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

pub fn get_value(num: usize) -> String {
    match (num % 5, num % 3) {
        (0, 0) => "Fizzbuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => format!("{}", num).to_string(),
    }
}

// fn get_vector(num: usize) -> Vec<String> {
    // let mut result: Vec<String> = Vec::new();
    // for i in 0..num {
        // result.push(match (i % 3, i % 5) {
            // (0, 0) => "Fizzbuzz".into(),
            // (0, _) => "Fizz".into(),
            // (_, 0) => "Buzz".into(),
            // (_, _) => i.to_string(),
        // });
    // }

    // result
// }
