use advent_of_code_2023::read_file;

mod day1;
mod day1_take2;

fn main() {
    let file = read_file("input", 1);
    let lines = file.lines().collect::<Vec<_>>();
    let result = day1_take2::pt_2(lines);
    println!("{}", result)
}
