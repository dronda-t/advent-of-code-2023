use std::collections::{BTreeMap, HashMap, HashSet};

fn parse_number_string(string: &str) -> HashSet<u32> {
    string.split(' ')
        .filter(|&chars| !chars.trim().is_empty() )
        .filter_map(|nums| nums.parse::<u32>().ok())
        .collect()
}

pub fn part_1(input: Vec<&str>) -> u32 {
    input.iter().map(|&line| {
        let (winning_numbers, my_numbers) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning_numbers = parse_number_string(winning_numbers);
        let my_numbers = parse_number_string(my_numbers);

        let winners = winning_numbers.intersection(&my_numbers).count();
        if winners > 0 {
            2_u32.pow((winners - 1) as u32)
        } else {
            0
        }
    }).sum()
}

pub fn part_2(input: Vec<&str>) -> u32 {
    let mut score_card_count = HashMap::new();
    input.iter().enumerate().for_each(|(index, &line)| {
        let (winning_numbers, my_numbers) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning_numbers = parse_number_string(winning_numbers);
        let my_numbers = parse_number_string(my_numbers);

        let winners = winning_numbers.intersection(&my_numbers).count();
        score_card_count.entry(index).and_modify(|count| *count += 1).or_insert(1);
        let current_value  = score_card_count.get(&index).unwrap().clone();
        for num in 1..=winners {
            score_card_count.entry(index + num).and_modify(|count| *count += current_value).or_insert(current_value);
        }
    });
    score_card_count.values().sum()
}

mod test {
    use std::time::Instant;
    use advent_of_code_2023::read_file;
    use crate::day4;

    #[test]
    fn test_part_1() {
        let file = read_file("input", 4);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day4::part_1(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }

    #[test]
    fn test_part_2() {
        let file = read_file("input", 4);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day4::part_2(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }

}
