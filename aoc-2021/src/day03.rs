use std::fs;

pub fn run() {
    println!("Day 3a: {}", day3a("assets/03.txt"));
    println!("Day 3b: {}", day3b("assets/03.txt"));
}

pub fn day3a(file_name: &str) -> String {
    let data: Vec<Vec<usize>> = parse_file(file_name);

    let int_length: usize = data[0].len();
    let threshold: usize = data.len() / 2;
    let mut bit_count: Vec<usize> = vec![0; int_length];

    for num in &data {
        for (i, c) in num.iter().enumerate() {
            bit_count[i] += c
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    for num in &bit_count {
        gamma <<= 1;
        epsilon <<= 1;
        if num > &threshold {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    (gamma * epsilon).to_string()
}

pub fn day3b(file_name: &str) -> String {
    parse_file(file_name);
    "tmp".to_string()
}

fn parse_file(file_name: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(file_name)
        .expect("Could not load file")
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| if n == '1' { 1 } else { 0 })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_3a() {
        assert_eq!(super::day3a("assets/03_test.txt"), "198");
    }
    #[test]
    fn part_3b() {
        assert_eq!(super::day3b("assets/03_test.txt"), "230");
    }
}
