use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn xy_in_grid(&self, x: isize, y: isize) -> bool {
        usize::try_from(y).ok().and_then(|y| self.0.get(y)).map(|row|{
            x < row.len() as isize
        }).unwrap_or(false)
    }

    fn create_valid_point(&self, x: isize, y: isize) -> Option<Point> {
        if self.xy_in_grid(x, y) {
            Some(Point { x, y })
        } else {
            None
        }
    }
    
    fn point(&self, point: &Point) -> Option<&char> {
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;

        let row = self.0.get(y)?;
        row.get(x)
    }
}

fn find_whole_number_direction<F>(grid: &Grid, point: &Point, visited: &mut HashSet<Point>, numbers: &mut Vec<u32>, update: F)
    where F: Fn(&Point) -> Point {
    let new_point = update(point);

    if let Some(digit) = grid.point(&new_point).and_then(|&char| char.to_digit(10)) {
        numbers.push(digit);
        visited.insert(new_point);
        find_whole_number_direction(grid, &new_point, visited, numbers, update)
    };
}

fn find_whole_number(grid: &Grid, visited: &mut HashSet<Point>, point: &Point) -> Option<u32> {
    if visited.contains(point) { return None }
    grid.point(point)
        .and_then(|char| char.to_digit(10))
        .map(|num|{
            let mut left = Vec::new();
            find_whole_number_direction(grid, point, visited, &mut left, |old| Point { x: old.x - 1, y: old.y });
            let mut right = Vec::new();
            find_whole_number_direction(grid, point, visited, &mut right, |old| Point { x: old.x + 1, y: old.y});

            left.reverse();
            left.push(num);
            left.extend(right);
            left.iter().fold(0, |acc, ele| acc * 10 + ele)
        })
}

fn find_valid_neighbors(grid: &Grid, visited: &mut HashSet<Point>, point: &Point) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    visited.insert(*point);
    if let Some(new_point) = grid.create_valid_point(point.x - 1, point.y) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|value| nums.push(*value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x - 1, point.y - 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x, point.y - 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x + 1, point.y - 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x + 1, point.y) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x + 1, point.y + 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x, point.y + 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }
    if let Some(new_point) = grid.create_valid_point(point.x - 1, point.y + 1) {
        find_whole_number(grid, visited, &new_point).iter().for_each(|&value| nums.push(value) )
    }

    nums
}

fn create_grid(input: Vec<&str>) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    input.iter().for_each(|line|{
        let mut row = Vec::with_capacity(input[0].chars().count());
        line.chars().for_each(|char| row.push(char));
        grid.push(row)
    });
    grid
}

mod part_1 {
    use std::collections::HashSet;
    use crate::day3::{create_grid, find_valid_neighbors, Grid, Point};

    fn breadth_first_search(grid: Grid) -> u32 {
        let mut visited = HashSet::new();
        grid.0.iter().enumerate().flat_map(|(y, row)|{
            row.iter().enumerate().filter_map(|(x, char)|{
                if !char.is_numeric() && *char != '.' {
                    let point = Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() };
                    Some(find_valid_neighbors(&grid, &mut visited, &point).iter().sum())
                } else { None }
            }).collect::<Vec<u32>>()
        }).collect::<Vec<u32>>().iter().sum()
    }

    pub fn part_1(input: Vec<&str>) -> u32 {
        let grid = create_grid(input);
        breadth_first_search(Grid(grid))
    }
}

mod part_2 {
    use std::collections::HashSet;
    use crate::day3::{create_grid, find_valid_neighbors, Grid, Point};

    fn breadth_first_search(grid: Grid) -> u32 {
        let mut visited = HashSet::new();
        grid.0.iter().enumerate().flat_map(|(y, row)|{
            row.iter().enumerate().filter_map(|(x, char)|{
                if !char.is_numeric() && *char != '.' {
                    let point = Point { x: x.try_into().unwrap(), y: y.try_into().unwrap() };
                    if *grid.point(&point)? == '*' {
                        let neighbors = find_valid_neighbors(&grid, &mut visited, &point);
                        if neighbors.len() == 2 {
                            Some(neighbors.iter().product())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else { None }
            }).collect::<Vec<u32>>()
        }).collect::<Vec<u32>>().iter().sum()
    }

    pub fn part_2(input: Vec<&str>) -> u32 {
        let grid = create_grid(input);
        breadth_first_search(Grid(grid))
    }
}


mod test {
    use std::time::Instant;
    use advent_of_code_2023::read_file;
    use crate::day3;

    #[test]
    fn test_part_1() {
        let file = read_file("input", 3);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day3::part_1::part_1(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }

    #[test]
    fn test_part_2() {
        let file = read_file("input", 3);
        let lines = file.lines().collect::<Vec<_>>();
        let beg = Instant::now();
        let result = day3::part_2::part_2(lines);
        let end = Instant::now();
        println!("{:?}", (end - beg));
        println!("{}", result)
    }
}
