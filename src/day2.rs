
#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32
}

fn parse_game(line: &str) -> Game {
    let (id, game_str) = line.split_once(' ')
        .unwrap().1
        .split_once(':')
        .unwrap();

    let id = id.parse::<u32>().expect("failed to parse");

    let mut game = Game {
        id,
        red: 0,
        green: 0,
        blue: 0
    };

    let rounds: Vec<_> = game_str.split(';').collect();
    for round in rounds {
        let colors: Vec<_> = round.split(',').collect();
        for quant_and_color in colors {
            let quant_and_color = quant_and_color.strip_prefix(' ').unwrap()
                .split(' ')
                .collect::<Vec<_>>();

            let mut quant_and_color = quant_and_color.iter();
            let quantity = quant_and_color.next().unwrap().parse::<u32>().unwrap();
            match quant_and_color.next().unwrap().to_owned() {
                "red" => { if quantity > game.red { game.red = quantity } }
                "green" => { if quantity > game.green { game.green = quantity } }
                "blue" => { if quantity > game.blue { game.blue = quantity } }
                _ => panic!("unrecognized string")
            }
        }
    }

    game
}

pub fn part_1(input: Vec<&str>) -> u32 {
    input.iter()
        .filter(|&line| !line.trim().is_empty())
        .map(|&line| parse_game(line))
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.id)
        .sum()
}

pub fn part_2(input: Vec<&str>) -> u32 {
    input.iter()
        .filter(|&line| !line.trim().is_empty())
        .map(|&line| parse_game(line))
        .map(|game| game.red  * game.blue * game.green)
        .sum()
}

mod test {
    use std::time::Instant;
    use advent_of_code_2023::read_file;
    use crate::day2;

    #[test]
    fn test_part_1() {
        let file = read_file("input", 2);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day2::part_1(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }

    #[test]
    fn test_part_2() {
        let file = read_file("input", 2);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day2::part_2(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }

}
