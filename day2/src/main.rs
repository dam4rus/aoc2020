#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new("^([\\d]+)-([\\d]+) ([a-z]): ([a-z]+)$").unwrap();
}

pub struct Password {
    pub char_policy_range: RangeInclusive<u32>,
    pub char_policy: char,
    pub password: String,
}

impl Password {
    pub fn from_input<T: AsRef<str>>(input: T) -> Option<Self> {
        let captures = INPUT_RE.captures(input.as_ref())?;
        let range_start = captures[1].parse::<u32>().ok()?;
        let range_end = captures[2].parse::<u32>().ok()?;
        Some(Self {
            char_policy_range: range_start..=range_end,
            char_policy: captures[3].chars().next()?,
            password: captures[4].to_owned(),
        })
    }

    pub fn char_count_in_range(&self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|c| *c == self.char_policy)
            .count();
        self.char_policy_range.contains(&(char_count as u32))
    }

    pub fn is_valid(&self) -> bool {
        let first_char = self
            .password
            .chars()
            .nth(*self.char_policy_range.start() as usize - 1)
            .expect("Password start index out of range");
        let second_char = self
            .password
            .chars()
            .nth(*self.char_policy_range.end() as usize - 1)
            .expect("Password end index out of range");
        (first_char == self.char_policy) ^ (second_char == self.char_policy)
    }
}

fn part1(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|password| password.char_count_in_range())
        .count()
}

fn part2(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|password| password.is_valid())
        .count()
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let buf_reader = BufReader::new(file);
    let passwords = buf_reader
        .lines()
        .map(|line| Password::from_input(line.ok()?))
        .collect::<Option<Vec<_>>>()
        .expect("Failed to parse input");

    println!("Valid passwords in part 1 count: {}", part1(&passwords));
    println!("Valid passwords in part 2 count: {}", part2(&passwords));
}
