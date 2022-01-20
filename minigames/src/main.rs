mod fizzbuzz;

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: usize = input
        .trim()
        .parse()
        .expect("Input was not a number");

    fizzbuzz::print_up_to(input);

    let val: String = fizzbuzz::get_value(input);

    println!("------------");
    println!("{}", val);


}
