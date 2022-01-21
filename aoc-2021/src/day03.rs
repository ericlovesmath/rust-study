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
    let data: Vec<Vec<usize>> = parse_file(file_name);
    let oxygen_gen_rating: usize = get_oxygen_generator_rating(&data);
    let c02_scrubber_rating: usize = get_c02_scrubber_rating(&data);

    (oxygen_gen_rating * c02_scrubber_rating).to_string()
}

enum RatingMode {
    OxygenGenerator,
    C02Scrubber,
}

fn get_oxygen_generator_rating(data: &Vec<Vec<usize>>) -> usize {
    get_rating(&data, RatingMode::OxygenGenerator)
}

fn get_c02_scrubber_rating(data: &Vec<Vec<usize>>) -> usize {
    get_rating(&data, RatingMode::C02Scrubber)
}

fn get_rating(original_data: &Vec<Vec<usize>>, mode: RatingMode) -> usize {
    let mut data = original_data.clone();
    let mut one_count: usize;
    let mut retain_num: usize;

    for i in 0..data[0].len() {
        one_count = data.iter().map(|n| n[i]).sum();

        retain_num = match mode {
            RatingMode::OxygenGenerator => one_count > (data.len() - 1) / 2,
            RatingMode::C02Scrubber => one_count < (data.len() + 1) / 2,
        } as usize;

        data.retain(|n| n[i] == retain_num);

        if data.len() == 1 {
            break;
        };
    }

    assert!(data.len() == 1);
    bit_array_to_int(&data[0])
}

fn bit_array_to_int(arr: &Vec<usize>) -> usize {
    let mut ans: usize = 0;
    for i in 0..arr.len() {
        ans <<= 1;
        ans += arr[i];
    }
    ans
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
