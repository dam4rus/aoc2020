use std::{
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Square {
    Open,
    Tree,
}

impl TryFrom<char> for Square {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Square::Open),
            '#' => Ok(Square::Tree),
            _ => Err("invalid square character"),
        }
    }
}

fn tree_count_with_slope(input: &Vec<Vec<Square>>, right_step: usize, down_step: usize) -> usize {
    input
        .iter()
        .step_by(down_step)
        .enumerate()
        .filter(
            |(i, row)| match row.iter().cycle().nth(i * right_step).unwrap() {
                Square::Tree => true,
                _ => false,
            },
        )
        .count()
}

fn part1(input: &Vec<Vec<Square>>) {
    println!(
        "Trees encounted with slope right 3 down 1: {}",
        tree_count_with_slope(input, 3, 1)
    );
}

fn part2(input: &Vec<Vec<Square>>) {
    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right_step, down_step)| tree_count_with_slope(input, *right_step, *down_step))
        .product::<usize>();
    println!("Tree encountered with all slopes: {}", result);
}

fn main() {
    let input_file = File::open("input.txt").expect("Failed to open input.txt");
    let input = BufReader::new(input_file)
        .lines()
        .map(|line| {
            line.expect("Invalid line")
                .chars()
                .map(Square::try_from)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    part1(&input);
    part2(&input);
}
