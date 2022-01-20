use std::fs;

pub fn run() {
    println!("Day 1a: {}", day1a("assets/01.txt"));
    println!("Day 1b: {}", day1b("assets/01.txt"));
}

pub fn day1a(file_name: &str) -> String {
    parse_file(file_name)
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
        .to_string()
}

pub fn day1b(file_name: &str) -> String {
    parse_file(file_name)
        .windows(4)
        .filter(|w| w[0] < w[3])
        .count()
        .to_string()
}

fn parse_file(file_name: &str) -> Vec<u32> {
    fs::read_to_string(file_name)
        .expect("Could not load file")
        .lines()
        .map(|l| l.parse::<u32>().expect("Data failed to parse to u32"))
        .collect::<Vec<u32>>()
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_1a() {
        assert_eq!(super::day1a("assets/01_test.txt"), "7");
    }
    #[test]
    fn part_1b() {
        assert_eq!(super::day1b("assets/01_test.txt"), "5");
    }
}
