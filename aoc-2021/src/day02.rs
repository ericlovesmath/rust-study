use std::fs;

pub fn run() {
    println!("Day 2a: {}", day2a("assets/02.txt"));
    println!("Day 2b: {}", day2b("assets/02.txt"));
}

pub fn day2a(file_name: &str) -> String {

    let data = parse_file(file_name);

    let mut distance: u32 = 0;
    let mut depth: u32 = 0;

    for (curr_direction, travel_length) in &data {
        match curr_direction.as_str() {
            "forward" => distance += travel_length,
            "down" => depth += travel_length,
            "up" => depth -= travel_length,
            _ => unreachable!("Direction check failed"),
        }
    }

    (distance * depth).to_string()
}

pub fn day2b(file_name: &str) -> String {

    let data = parse_file(file_name);

    let mut distance: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for (curr_direction, travel_length) in &data {
        match curr_direction.as_str() {
            "forward" => {
                distance += travel_length;
                depth += aim * travel_length;
            }
            "down" => aim += travel_length,
            "up" => aim -= travel_length,
            _ => unreachable!("Direction check failed"),
        }
    }

    (distance * depth).to_string()
}

fn parse_file(file_name: &str) -> Vec<(String, u32)> {
    fs::read_to_string(file_name)
        .expect("Could not load file")
        .lines()
        .map(|line| {
            let pair: (&str, &str) = line.split_once(' ').unwrap();
            (pair.0.to_owned(), pair.1.parse::<u32>().unwrap())
        })
        .collect::<Vec<(String, u32)>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1a() {
        assert_eq!(super::day2a("assets/02_t1.txt"), "150");
    }
}
