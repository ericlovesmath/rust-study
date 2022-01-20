use std::fs;

pub fn run() {
    println!("Day 3a: {}", day3a("assets/03.txt"));
    println!("Day 3b: {}", day3b("assets/03.txt"));
}

pub fn day3a(file_name: &str) -> String {
    let data: Vec<String> = parse_file(file_name);

    let int_length: usize = data[0].len();
    let threshold: usize = data.len() / 2;
    let mut bit_count: Vec<usize> = vec![0; int_length];

    for num in data {
        for (i, c) in num.chars().enumerate() {
            if c == '1' {
                bit_count[i] += 1
            }
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

    // println!("{:?}", bit_count);
    // println!("{}, {}", gamma, epsilon);

    (gamma * epsilon).to_string()
}

pub fn day3b(file_name: &str) -> String {
    parse_file(file_name);
    "tmp".to_string()
}

fn parse_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name)
        .expect("Could not load file")
        .lines()
        // .map(|l| l.parse::<u33>().expect("Data failed to parse to u32"))
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_3a() {
        assert_eq!(super::day3a("assets/03_test.txt"), "198");
    }
    // #[test]
    // fn part_3b() {
    // assert_eq!(super::day3b("assets/03_test.txt"), "5");
    // }
}
