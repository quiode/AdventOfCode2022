use crate::{ line_manager::{ get_lines, self }, types::Lines };

pub fn main() {
    let lines = get_lines(line_manager::FILE);

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> i32 {
    let mut biggest_cal = 0;
    let mut current_cal = 0;

    for line in lines {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            if current_cal >= biggest_cal {
                biggest_cal = current_cal;
            }

            current_cal = 0;
        } else {
            let cal: i32 = line.parse().unwrap();
            current_cal += cal;
        }
    }

    biggest_cal
}

fn problem2(lines: Lines) -> i32 {
    let mut top3: [i32; 3] = [0; 3];

    let mut current_cal = 0;

    for line in lines {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            top3.sort();

            if top3[0] < current_cal {
                top3[0] = current_cal;
            }
            current_cal = 0;
        } else {
            current_cal += line.parse::<i32>().unwrap();
        }
    }

    top3.sort();

    if top3[0] < current_cal {
        top3[0] = current_cal;
    }

    let mut sum = 0;
    for i in top3 {
        sum += i;
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::line_manager::{ create_lines, TEST_FILE };

    use super::{ problem1, problem2 };

    #[test]
    fn problem1_test() {
        let result = problem1(create_lines("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
", TEST_FILE));
        assert_eq!(result, 24000);
    }

    #[test]
    fn problem2_test() {
        let result = problem2(create_lines("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
", TEST_FILE));
        assert_eq!(result, 45000);
    }
}