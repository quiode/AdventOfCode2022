use crate::{ types::{ Lines, DupVecDeque }, line_manager };

pub fn main() {
    let lines = crate::line_manager::get_lines(line_manager::FILE);

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
}

fn problem1(mut lines: Lines) -> usize {
    let text_string = lines.next().unwrap().unwrap();

    get_index_of_distinct(&text_string, 4)
}

fn problem2(mut lines: Lines) -> usize {
    let text_string = lines.next().unwrap().unwrap();

    get_index_of_distinct(&text_string, 14)
}

/// gets the index of the character when `amount_of_distinct` characters in a squence are distinct
fn get_index_of_distinct(char_buff: &str, amount_of_distinct: usize) -> usize {
    let mut count = 0;
    let mut queue = DupVecDeque::with_capacity(amount_of_distinct + 1);
    for char in char_buff.chars() {
        count += 1;
        let queue_vec = queue.vec_mut();
        queue_vec.push_back(char);
        if queue_vec.len() <= amount_of_distinct {
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

    use crate::{ line_manager::{ create_lines, TEST_FILE }, day6::problem2 };

    use super::problem1;

    #[test]
    fn problem1_test() {
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

    #[test]
    fn problem2_test() {
        let mut problem_solution = HashMap::new();
        problem_solution.insert("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        problem_solution.insert("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        problem_solution.insert("nppdvjthqldpwncqszvftbrmjlhg", 23);
        problem_solution.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        problem_solution.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);

        for problem in problem_solution {
            let result = problem2(create_lines(problem.0, TEST_FILE));

            assert_eq!(result, problem.1);
        }
    }
}