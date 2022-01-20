mod day01;
mod day02;
mod day03;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");

    match problem {
        "day01" => day01::run(),
        "day02" => day02::run(),
        "day03" => day03::run(),
        _ => println!("Haven't gotten to that yet!"),
    };
}
