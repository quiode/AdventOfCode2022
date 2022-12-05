use std::fs::File;
use std::io::{BufReader, Lines};

pub fn main() {
    let lines = crate::line_manager::get_lines("input.txt");

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines<BufReader<File>>) -> String {
    let mut stacks: Vec<Vec<char>> = vec![];
    // set of instructions, move i32 from usize to usize
    let mut instructions: Vec<(i32, usize, usize)> = vec![];

    // parse input
    for line in lines {
        let line = line.unwrap();
        // if line contains [ it is a stack line
        if line.contains("[") {
            let line_chars = line.chars().collect::<Vec<_>>();

            for (i, char) in line_chars.iter().enumerate() {
                if ((i as i32) - 1) % 4 == 0 && ('A'..='Z').contains(char) {
                    let index = (((i as f32) + 1.0) / 4.0).floor() as usize;
                    if let None = stacks.get(index){
                        for i in 0..=index{
                            if let None = stacks.get(i){
                                stacks.push(vec![]);
                            }
                        }
                    }
                    stacks[index].insert(0,*char);
                }
            }
        } else if line.contains("move") {
            let splits = line.split(" ").collect::<Vec<_>>();
            let instruction: (i32, usize, usize) = (splits[1].parse().unwrap(), splits[3].parse::<usize>().unwrap() - 1, splits[5].parse::<usize>().unwrap() - 1);
            instructions.push(instruction);
        }
    }

    // works until here

    // execute instructions
    for instruction in instructions {
        for _ in 0..instruction.0 {
            if let Some(item) = stacks[instruction.1].pop() {
                stacks[instruction.2].push(item);
            }
        }
    }

    let mut top_items = String::new();
    for stack in stacks {
        if let Some(top_item) = stack.last() {
            top_items.push(*top_item);
        }
    }
    top_items
}

#[cfg(test)]
mod tests {
    use crate::day5::problem1;
    use crate::line_manager::{create_lines, TEST_FILE};

    #[test]
    fn problem1_test() {
        let solution = problem1(create_lines("    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
", TEST_FILE));

        assert_eq!(solution, "CMZ");
    }
}