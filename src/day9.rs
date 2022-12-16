use crate::{ types::Lines, line_manager };

use self::imp::{ read_instructions, Helper };

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

fn problem1(lines: Lines) -> usize {
    let mut helper = Helper::new();
    let instructions = read_instructions(lines);

    helper.execute_instructions(instructions);

    helper.count_positions()
}

fn problem2(lines: Lines) -> u32 {
    todo!()
}

mod imp {
    use std::{ collections::HashSet };

    use crate::{ types::Lines, shared::* };

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

    pub struct Helper {
        tail: Coordinates,
        head: Coordinates,
        tail_positions: HashSet<Coordinates>,
    }

    impl Helper {
        pub fn new() -> Self {
            let mut hash_set = HashSet::new();
            hash_set.insert((0, 0));
            Self { tail: (0, 0), head: (0, 0), tail_positions: hash_set }
        }

        /// moves the head into the desired direction, if the tail is too far away, also move tail
        fn move_head(&mut self, direction: Direction) {
            match direction {
                Direction::UP => {
                    self.head.1 += 1;
                }
                Direction::DOWN => {
                    self.head.1 -= 1;
                }
                Direction::LEFT => {
                    self.head.0 -= 1;
                }
                Direction::RIGHT => {
                    self.head.0 += 1;
                }
            }

            if self.check_tail() {
                return;
            }

            // moves tail near head
            self.move_tail((
                (self.head.0 - self.tail.0).signum(),
                (self.head.1 - self.tail.1).signum(),
            ));
        }

        /// checks if the tail is still in reach of the head or not
        fn check_tail(&self) -> bool {
            // check left, right, up, down
            !((self.head.0 - self.tail.0).abs() >= 2 || (self.head.1 - self.tail.1).abs() >= 2)
        }

        /// moves tail to direction and logs new position
        fn move_tail(&mut self, direction: (i32, i32)) {
            self.tail.0 += direction.0;
            self.tail.1 += direction.1;

            self.tail_positions.insert(self.tail);
        }

        /// executes an instruction
        fn execute_instruction(&mut self, instruction: (Direction, usize)) {
            for _ in 0..instruction.1 {
                self.move_head(instruction.0);
            }
        }
        /// counts how on how many positions the tail has been in
        pub fn count_positions(&self) -> usize {
            self.tail_positions.len()
        }

        /// executes all instructions
        pub fn execute_instructions(&mut self, instructions: Vec<(Direction, usize)>) {
            for instruction in instructions {
                self.execute_instruction(instruction);
            }
        }

        /// prints the grid
        pub fn print(&self) {
            let smallest_x = vec![
                self.tail.0,
                self.head.0,
                self.tail_positions
                    .iter()
                    .min_by(|x, y| x.0.cmp(&y.0))
                    .unwrap().0
            ]
                .iter()
                .min()
                .unwrap()
                .to_owned();

            let biggest_x = vec![
                self.tail.0,
                self.head.0,
                self.tail_positions
                    .iter()
                    .max_by(|x, y| x.0.cmp(&y.0))
                    .unwrap().0
            ]
                .iter()
                .max()
                .unwrap()
                .to_owned();

            let smallest_y = vec![
                self.tail.1,
                self.head.1,
                self.tail_positions
                    .iter()
                    .min_by(|x, y| x.1.cmp(&y.1))
                    .unwrap().1
            ]
                .iter()
                .min()
                .unwrap()
                .to_owned();

            let biggest_y = vec![
                self.tail.1,
                self.head.1,
                self.tail_positions
                    .iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap().1
            ]
                .iter()
                .max()
                .unwrap()
                .to_owned();

            for y in (smallest_y..=biggest_y).rev() {
                for x in smallest_x..=biggest_x {
                    let mut printed = false;
                    print!(" ");
                    if self.tail == (x, y) {
                        printed = true;
                        print!("T");
                    }
                    if self.head == (x, y) {
                        printed = true;
                        print!("H");
                    }
                    if self.tail_positions.contains(&(x, y)) {
                        printed = true;
                        print!("#");
                    }
                    if !printed {
                        print!(".");
                    }
                    print!(" ");
                }

                println!();
            }
        }
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