mod types;

use crate::{ types::Lines, line_manager };

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> i32 {
    todo!()
}

fn problem2(lines: Lines) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::line_manager::{ create_lines, TEST_FILE };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let result = problem1(create_lines("30373
25512
65332
33549
35390", TEST_FILE));

        assert_eq!(result, 21);
    }

    #[test]
    fn problem2_test() {
        todo!()
    }
}