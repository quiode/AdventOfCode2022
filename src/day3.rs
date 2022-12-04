use std::{ io::{ Lines, BufReader }, fs::File, collections::HashMap };

use crate::line_manager;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;
    let values: HashMap<char, i32> = get_values();

    for line in lines {
        let line = line.unwrap();

        let (first, second) = line.split_at(line.len() / 2);
        let duplicate = first
            .chars()
            .find(|char| second.chars().any(|schar| schar == *char))
            .unwrap();

        sum += values.get(&duplicate).unwrap();
    }

    sum
}

fn problem2(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;
    let values = get_values();

    let mut lines_iterator = lines.into_iter();

    while let Ok(group) = lines_iterator.next_chunk::<3>() {
        let group = group.map(|line| line.unwrap().chars().collect::<Vec<_>>());

        let overlapping = find_overlapping(&group[0], &group[1]);
        let overlapping = find_overlapping(&overlapping, &group[2]);

        if overlapping.len() != 1 {
            panic!("Only one character overlapps!");
        }

        let overlapping_char = overlapping[0] as char;

        sum += values.get(&overlapping_char).unwrap();
    }

    sum
}

/// finds overlapping chars from two inputs
fn find_overlapping(chars1: &Vec<char>, chars2: &Vec<char>) -> Vec<char> {
    let mut overlapping = vec![];

    for char in chars1.clone().into_iter() {
        if
            chars2
                .clone()
                .into_iter()
                .any(|char2| char == char2) &&
            !overlapping.contains(&char)
        {
            overlapping.push(char);
        }
    }

    overlapping
}

fn get_values() -> HashMap<char, i32> {
    let mut values: HashMap<char, i32> = HashMap::new();
    for (i, c) in ('a'..='z').enumerate() {
        values.insert(c, (i + 1).try_into().unwrap());
    }
    for (i, c) in ('A'..='Z').enumerate() {
        values.insert(c, (i + 27).try_into().unwrap());
    }

    values
}

#[cfg(test)]
mod tests {
    use crate::{ line_manager, day3::problem2 };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let lines = line_manager::create_lines(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
            line_manager::TEST_FILE
        );

        assert_eq!(problem1(lines), 157);
    }

    #[test]
    fn problem2_test() {
        let lines = line_manager::create_lines(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
            line_manager::TEST_FILE
        );

        assert_eq!(problem2(lines), 70);
    }
}