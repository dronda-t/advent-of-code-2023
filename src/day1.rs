use std::collections::{HashMap, HashSet};
use regex::{Match, Regex};

fn first_digit(chars: &str) -> Option<u32> {
    for char in chars.chars() {
        if char.is_digit(10) {
            return Some(char.to_digit(10).unwrap())
        }
    }
    None
}

pub fn part1(input: Vec<&str>) -> u32 {
    return input.iter().map(|&line| {
        let first = first_digit(&line).unwrap();
        let last = first_digit(&line.chars().rev().collect::<String>()).unwrap();
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum()
}

fn first_digit_index(string: &str) -> Option<usize> {
    for (i, ch) in string.chars().enumerate() {
        if ch.is_digit(10) {
            return Some(i);
        }
    }
    None
}

#[derive(Debug)]
struct WordIndex {
    word: String,
    index_in_str: usize
}

impl WordIndex {
    fn new(word: String, index_in_str: usize) -> WordIndex {
        WordIndex {
            word,
            index_in_str
        }
    }
}

fn get_index_of_words(chars: &str) -> u32 {
    let list = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let map: HashMap<&str, u32> = HashMap::from([("one", 1),("two", 2),("three",3),("four",4),("five",5), ("six",6),("seven",7),("eight",8),("nine",9)]);
    let word_Indexe = list.iter().enumerate()
        .filter_map(|(index, &word)| {
            if let Some(index_in_str) = chars.find(word) {
                Some(WordIndex::new(list[index].to_string(), index_in_str) )
            } else {
                None
            }
        }).min_by(|word_index: &WordIndex, word_index2: &WordIndex| word_index.index_in_str.cmp(&word_index2.index_in_str));
    let digit_index_option = first_digit_index(&chars);
    let value = if let (Some(word_index), Some(digit_index)) = (&word_Indexe, digit_index_option) {
        if digit_index < word_index.index_in_str {
            chars.chars().collect::<Vec<_>>().get(digit_index_option.unwrap()).unwrap().to_digit(10).unwrap()
        } else {
            map.get(word_index.word.as_str()).unwrap().to_owned()
        }
    } else if let Some(word_index) = &word_Indexe {
        map.get(word_index.word.as_str()).unwrap().to_owned()
    } else if let Some (digit_index) = digit_index_option {
        chars.chars().collect::<Vec<_>>().get(digit_index_option.unwrap()).unwrap().to_digit(10).unwrap()
    } else {
        panic!("Shouldn't get here")
    };

    return value;
}

fn get_index_of_words_reversed(chars: &str) -> u32 {
    let list = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let map: HashMap<&str, u32> = HashMap::from([("one", 1),("two", 2),("three",3),("four",4),("five",5), ("six",6),("seven",7),("eight",8),("nine",9)]);
    let word_Indexe = list.iter().enumerate()
        .filter_map(|(index, &word)| {
            if let Some(index_in_str) = chars.rfind(word) {
                Some(WordIndex::new(list[index].to_string(), index_in_str) )
            } else {
                None
            }
        }).max_by(|word_index: &WordIndex, word_index2: &WordIndex| word_index.index_in_str.cmp(&word_index2.index_in_str));
    let digit_index_option = first_digit_index(&chars.chars().rev().collect::<String>())
        .map(|index| chars.chars().count() - index - 1);
    let value = if let (Some(word_index), Some(digit_index)) = (&word_Indexe, digit_index_option) {
        // println!("{:?}{}", word_index, digit_index);
        if digit_index > word_index.index_in_str {
            chars.chars().collect::<Vec<_>>().get(digit_index_option.unwrap()).unwrap().to_digit(10).unwrap()
        } else {
            map.get(word_index.word.as_str()).unwrap().to_owned()
        }
    } else if let Some(word_index) = &word_Indexe {
        map.get(word_index.word.as_str()).unwrap().to_owned()
    } else if let Some (digit_index) = digit_index_option {
        chars.chars().collect::<Vec<_>>().get(digit_index_option.unwrap()).unwrap().to_digit(10).unwrap()
    } else {
        panic!("Shouldn't get here")
    };

    return value;
}

fn get_index_of_reversed_string(str: &str, index: &usize) -> char {
    let count = str.chars().count();
    str.chars().collect::<Vec<_>>().get(count - index - 1).unwrap().to_owned()
}

// fn get_string_value(chars: &str) -> u32 {
//
//
//     let mut regex_iter = regex.find_iter(chars);
//     let first = regex_iter.next().unwrap();
//     let second : Match= regex_iter.last().unwrap_or(first);
//     println!("{} {}", first.as_str(), second.as_str());
//     let first_digit = get_digit_or_parse_digit_string(&map, first);
//     let last_digit = get_digit_or_parse_digit_string(&map, second);
//
//     format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap()
// }

fn get_digit_or_parse_digit_string(map: &HashMap<&str, u32>, first: Match) -> u32 {
    let first_digit = if let Ok(digit) = first.as_str().parse::<u32>() {
        digit
    } else {
        map.get(first.as_str()).unwrap().to_owned()
    };
    first_digit
}

pub fn part2(input: Vec<&str>) -> u32 {
    input.iter().map(|&line| {
        let first = get_index_of_words(line);
        let last = get_index_of_words_reversed(line);
        // println!("{} {}", first, last);
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum()
}

// 53592

mod test {
    use std::time::Instant;
    use advent_of_code_2023::read_file;
    use crate::day1;

    #[test]
    fn test_part_2() {
        let file = read_file("input", 1);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day1::part2(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }
}