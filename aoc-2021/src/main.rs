mod day01;
mod utils;

fn main() {
    // cargo run day01
    // cargo test day01
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    match problem {
        "day01" => day01::run(),
        _ => println!("Haven't gotten to that yet!"),
    };
}
