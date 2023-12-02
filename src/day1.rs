use std::collections::HashMap;
use once_cell::sync::Lazy;
use regex::Regex;

static WORD_VALUES: Lazy<HashMap<&str, u32>> = Lazy::new(|| HashMap::from([("one", 1),("two", 2),("three",3),("four",4),("five",5), ("six",6),("seven",7),("eight",8),("nine",9)]));
static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap());


pub fn pt_2(input: Vec<&str>) -> u32 {
    input.iter().map(|&line|{
        let values = (0..line.len()).filter_map(|index|{
            REGEX.find(&line[index..])
        }).map(|result|{
            if let Some(&word_value) = WORD_VALUES.get(result.as_str()) {
                word_value
            } else {
                result.as_str().parse().unwrap()
            }
        }).collect::<Vec<u32>>();

        let first = values.first().unwrap();
        let last = values.last().unwrap();
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum()
}

mod test {
    use std::time::Instant;
    use advent_of_code_2023::read_file;
    use crate::day1;

    #[test]
    fn test_part_2() {
        let file = read_file("input", 1);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day1::pt_2(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }
}