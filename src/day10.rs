use crate::{ types::Lines, line_manager };

use self::imp::{ State, get_instructions, execute_instruction, Instruction };

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> i32 {
    let mut sum = 0;
    let mut state = State::default();
    let instructions = get_instructions(lines);

    for instruction in instructions {
        if
            (state.get_cycle() + 20) % 40 == 0 ||
            (matches!(instruction, Instruction::addx(_)) &&
                (state.get_cycle() + 21) % 40 < (state.get_cycle() + 20) % 40)
        {
            let xth_cycle = (state.get_cycle() + 24) / 40;

            sum += ((xth_cycle as i32) * 40 - 20) * state.get_xval();
        }

        execute_instruction(&mut state, instruction);
    }

    return sum;
}

fn problem2(lines: Lines) -> u32 {
    todo!()
}

pub mod imp {
    use crate::types::Lines;

    pub fn get_instructions(lines: Lines) -> Vec<Instruction> {
        let mut instructions = vec![];

        for line in lines {
            let line = line.unwrap();

            if line == "noop" {
                instructions.push(Instruction::noop);
            } else {
                let value = line.split(" ").nth(1).unwrap();

                let int_value: i32 = value.parse().unwrap();

                instructions.push(Instruction::addx(int_value));
            }
        }

        instructions
    }

    pub fn execute_instruction(state: &mut State, instruction: Instruction) {
        match instruction {
            Instruction::noop => {
                state.cycle += 1;
            }
            Instruction::addx(x) => {
                state.cycle += 2;
                state.xval += x;
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub enum Instruction {
        addx(i32),
        noop,
    }

    pub struct State {
        xval: i32,
        cycle: usize,
    }

    impl Default for State {
        fn default() -> Self {
            Self { xval: 1, cycle: 1 }
        }
    }

    impl State {
        pub fn get_xval(&self) -> i32 {
            self.xval
        }

        pub fn get_cycle(&self) -> usize {
            self.cycle
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::line_manager::{ create_lines, TEST_FILE };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let input =
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let lines = create_lines(input, TEST_FILE);

        let solution = problem1(lines);

        assert_eq!(13140, solution);
    }

    #[test]
    fn problem2_test() {
        todo!()
    }
}