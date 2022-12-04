use std::{ fs::File, io::{ BufReader, Lines } };

use crate::line_manager;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);
    let solution = problem1(lines);
    println!("{}", solution);
}

fn problem1(lines: Lines<BufReader<File>>) -> i32 {
    let mut score = 0;

    for line in lines {
        let line = line.unwrap();
        if line.len() != 3 {
            panic!("Line length is not 3!");
        }

        let chars: Vec<char> = line.chars().collect();
        let (enemy, own) = (&chars[0], &chars[2]);
        let wins = wins(enemy, own);
        let shape_score = match own {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Value not allowed!"),
        };

        score += wins + shape_score;
    }

    score
}

/// calculates who wins in rock, paper, scissors
/// returns 0 if lost, 3 if draw and 6 if won
fn wins(enemy_val: &char, own_val: &char) -> i32 {
    match enemy_val {
        'A' => {
            match own_val {
                'X' => {
                    return 3;
                }
                'Y' => {
                    return 6;
                }
                'Z' => {
                    return 0;
                }
                _ => panic!("Value not allowed!"),
            }
        }
        'B' => {
            match own_val {
                'X' => {
                    return 0;
                }
                'Y' => {
                    return 3;
                }
                'Z' => {
                    return 6;
                }
                _ => panic!("Value not allowed!"),
            }
        }
        'C' => {
            match own_val {
                'X' => {
                    return 6;
                }
                'Y' => {
                    return 0;
                }
                'Z' => {
                    return 3;
                }
                _ => panic!("Value not allowed!"),
            }
        }
        _ => panic!("Value not allowed!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::line_manager::{ create_lines, TEST_FILE };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let result = problem1(create_lines("A Y
B X
C Z
", TEST_FILE));

        assert_eq!(result, 15);
    }
}