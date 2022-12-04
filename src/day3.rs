use std::{ io::{ Lines, BufReader }, fs::File, collections::HashMap };

use crate::line_manager;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
}

fn problem1(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    let mut values: HashMap<char, i32> = HashMap::new();
    for (i, c) in ('a'..='z').enumerate() {
        values.insert(c, (i + 1).try_into().unwrap());
    }
    for (i, c) in ('A'..='Z').enumerate() {
        values.insert(c, (i + 27).try_into().unwrap());
    }

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

#[cfg(test)]
mod tests {
    use crate::line_manager;

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
}