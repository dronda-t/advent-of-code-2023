
#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32
}

fn parse_game(line: &str) -> Game {
    let mut id_and_game = line.splitn(2, ' ')
        .last()
        .unwrap()
        .splitn(2, ':');

    let id = id_and_game.next().expect("no next").parse::<u32>().expect("failed to parse");

    let mut game = Game {
        id: id,
        red: 0,
        green: 0,
        blue: 0
    };

    let rounds: Vec<_> = id_and_game.last().unwrap().split(';').collect();
    for round in rounds {
        let colors: Vec<_> = round.split(',').collect();
        for quant_and_color in colors {
            let vec = quant_and_color.split(' ').collect::<Vec<_>>();
            let mut quant_and_color_iter = vec.iter();
            quant_and_color_iter.next();
            let quantity = quant_and_color_iter.next().unwrap().parse::<u32>().unwrap();
            match quant_and_color_iter.next().unwrap().to_owned() {
                "red" => { if quantity > game.red { game.red = quantity } }
                "green" => { if quantity > game.green { game.green = quantity } }
                "blue" => { if quantity > game.blue { game.blue = quantity } }
                _ => panic!("unrecognized string")
            }
        }
    }

    return game;
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
