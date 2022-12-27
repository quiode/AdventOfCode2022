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

const TEST_2_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> usize {
    let mut helper = Helper::default();
    let instructions = read_instructions(lines);

    helper.execute_instructions(instructions);

    helper.count_positions()
}

fn problem2(lines: Lines) -> usize {
    let mut helper = Helper::new(10);
    let instructions = read_instructions(lines);

    helper.execute_instructions(instructions);

    helper.count_positions()
}

mod imp {
    use std::collections::HashSet;

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
        /// the blocks of the rope
        blocks: Vec<Coordinates>,
        tail_positions: HashSet<Coordinates>,
    }

    impl Helper {
        pub fn new(block_count: u32) -> Self {
            let mut hash_set = HashSet::new();
            hash_set.insert((0, 0));

            let mut blocks: Vec<Coordinates> = Vec::new();

            for _ in 0..block_count {
                blocks.push((0, 0));
            }

            Self {
                blocks,
                tail_positions: hash_set,
            }
        }

        /// moves a block in the desired direction, if the next block is too far away, also move the next block
        fn move_block(&mut self, direction: Coordinates, block_index: usize) {
            self.blocks[block_index].0 += direction.0;
            self.blocks[block_index].1 += direction.1;

            // if block is the tail block, add new position to tail positions
            if block_index == self.blocks.len() - 1 {
                self.tail_positions.insert(self.blocks[block_index]);
                return;
            }

            if self.check_child(block_index) {
                return;
            }

            // moves the next block
            self.move_block(
                (
                    (self.blocks[block_index].0 - self.blocks[block_index + 1].0).signum(),
                    (self.blocks[block_index].1 - self.blocks[block_index + 1].1).signum(),
                ),
                block_index + 1
            );
        }

        /// checks if the next block is still in reach of the previous block or not
        fn check_child(&self, block_index: usize) -> bool {
            // check left, right, up, down
            !(
                (self.blocks[block_index].0 - self.blocks[block_index + 1].0).abs() >= 2 ||
                (self.blocks[block_index].1 - self.blocks[block_index + 1].1).abs() >= 2
            )
        }

        /// executes an instruction
        fn execute_instruction(&mut self, instruction: (Direction, usize)) {
            for _ in 0..instruction.1 {
                self.move_block(instruction.0.into(), 0);
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
                self.tail_positions
                    .iter()
                    .min_by(|x, y| x.0.cmp(&y.0))
                    .unwrap().0,
                self.blocks
                    .iter()
                    .min_by(|x, y| x.0.cmp(&y.0))
                    .unwrap().0
            ]
                .iter()
                .min()
                .unwrap()
                .to_owned();

            let biggest_x = vec![
                self.blocks
                    .iter()
                    .max_by(|x, y| x.0.cmp(&y.0))
                    .unwrap().0,
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
                self.blocks
                    .iter()
                    .min_by(|x, y| x.1.cmp(&y.1))
                    .unwrap().1,
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
                self.blocks
                    .iter()
                    .max_by(|x, y| x.1.cmp(&y.1))
                    .unwrap().1,
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
                    for (index, block) in self.blocks.iter().enumerate() {
                        if block.0 == x && block.1 == y {
                            if index == 0 {
                                print!("H");
                            } else if index == self.blocks.len() - 1 {
                                print!("T");
                            } else {
                                print!("{index}");
                            }

                            printed = true;
                            break;
                        }
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

    impl Default for Helper {
        fn default() -> Self {
            let mut hash_set: HashSet<Coordinates> = HashSet::new();
            hash_set.insert((0, 0));

            Self {
                blocks: vec![(0, 0), (0, 0)],
                tail_positions: hash_set,
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
    use crate::{ line_manager::{ create_lines, TEST_FILE }, day9::{ TEST_INPUT, TEST_2_INPUT } };

    use super::{ problem1, problem2 };

    #[test]
    fn problem1_test() {
        let solution = problem1(create_lines(TEST_INPUT, TEST_FILE));

        assert_eq!(solution, 13);
    }

    #[test]
    fn problem2_test() {
        let solution = problem2(create_lines(TEST_2_INPUT, TEST_FILE));

        assert_eq!(solution, 36);
    }
}