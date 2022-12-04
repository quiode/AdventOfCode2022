use std::{ io::{ Lines, BufReader }, fs::File, ops::Range };

use crate::line_manager;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();
        let ranges = line.split(",").collect::<Vec<_>>();
        let range1 = parse_range(ranges[0]);
        let range2 = parse_range(ranges[1]);

        if contains(&range1, &range2) {
            sum += 1;
        }
    }

    sum
}

/// parses a string and returns the range
fn parse_range(range_string: &str) -> Range<i32> {
    let split = range_string.split("-").collect::<Vec<_>>();
    let start = split[0].parse::<i32>().unwrap();
    let end = split[1].parse::<i32>().unwrap();

    start..end
}

/// returs true if one range is contained in the other
fn contains(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    (range1.start <= range2.start && range1.end >= range2.end) ||
        (range1.start >= range2.start && range1.end <= range2.end)
}

#[cfg(test)]
mod tests {
    use crate::line_manager;

    use super::problem1;

    #[test]
    fn problem1_test() {
        let result = problem1(
            line_manager::create_lines(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
                line_manager::TEST_FILE
            )
        );

        assert_eq!(result, 2);
    }
}