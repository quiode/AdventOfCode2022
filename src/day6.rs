use crate::{ types::{ Lines, DupVecDeque }, line_manager };

pub fn main() {
    let lines = crate::line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(mut lines: Lines) -> i32 {
    let text_string = lines.next().unwrap().unwrap();

    let mut count = 0;
    let mut queue = DupVecDeque::with_capacity(5);
    for char in text_string.chars() {
        count += 1;
        let queue_vec = queue.vec_mut();
        queue_vec.push_back(char);
        if queue_vec.len() <= 4 {
            continue;
        }
        queue_vec.pop_front();
        if queue.has_duplicates() {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::line_manager::{ create_lines, TEST_FILE };

    use super::problem1;

    #[test]
    fn problem1_test_1() {
        let mut problem_solution = HashMap::new();
        problem_solution.insert("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        problem_solution.insert("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        problem_solution.insert("nppdvjthqldpwncqszvftbrmjlhg", 6);
        problem_solution.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        problem_solution.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

        for problem in problem_solution {
            let result = problem1(create_lines(problem.0, TEST_FILE));

            assert_eq!(result, problem.1);
        }
    }
}