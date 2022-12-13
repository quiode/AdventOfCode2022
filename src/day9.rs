use crate::{ types::Lines, line_manager };

const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> u32 {
    todo!()
}

fn problem2(lines: Lines) -> u32 {
    todo!()
}

mod imp {
    use crate::{ types::Lines, shared::Direction };

    pub fn read_instructions(lines: Lines) -> Vec<(Direction, usize)> {
        let mut instructions: Vec<(Direction, usize)> = vec![];

        for line in lines {
            let line = line.unwrap();
            let mut splits = line.split(" ");
            let dir = splits.next().unwrap();
            let times = splits.next().unwrap();

            let dir = match dir {
                "R" => Direction::RIGHT,
                "U" => Direction::UP,
                "L" => Direction::LEFT,
                "D" => Direction::DOWN,
                _ => panic!("Value not allowed as direction!"),
            };
            let times: usize = times.parse().unwrap();

            instructions.push((dir, times));
        }

        instructions
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            line_manager::{ create_lines, TEST_FILE },
            day9::TEST_INPUT,
            shared::Direction,
        };

        use super::read_instructions;

        #[test]
        fn read_instructions_test() {
            let lines = create_lines(TEST_INPUT, TEST_FILE);
            let exspected_result: Vec<(Direction, usize)> = vec![
                (Direction::RIGHT, 4),
                (Direction::UP, 4),
                (Direction::LEFT, 3),
                (Direction::DOWN, 1),
                (Direction::RIGHT, 4),
                (Direction::DOWN, 1),
                (Direction::LEFT, 5),
                (Direction::RIGHT, 2)
            ];

            let result = read_instructions(lines);

            for (i, e) in result.iter().enumerate() {
                assert_eq!(&exspected_result[i], e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ line_manager::{ create_lines, TEST_FILE }, day9::TEST_INPUT };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let solution = problem1(create_lines(TEST_INPUT, TEST_FILE));

        assert_eq!(solution, 13);
    }

    #[test]
    fn problem2_test() {
        todo!()
    }
}