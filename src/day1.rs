use std::{ fs::File, io::{ BufRead, BufReader, Lines } };

pub fn main() {
    let lines = get_lines("input.txt");

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines<BufReader<File>>) -> i32 {
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

fn problem2(lines: Lines<BufReader<File>>) -> i32 {
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

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let input_file = File::open(path).unwrap();
    BufReader::new(input_file).lines()
}

#[cfg(test)]
mod test {
    use std::{ fs::File, io::{ Write, self, BufRead, Lines, BufReader }, thread::Result };

    use super::{ problem1, get_lines, problem2 };

    #[test]
    fn problem1_test() {
        let result = problem1(get_sample_lines());
        assert_eq!(result, 24000);
    }

    #[test]
    fn problem2_test() {
        let result = problem2(get_sample_lines());
        assert_eq!(result, 45000);
    }

    fn get_sample_lines() -> Lines<BufReader<File>> {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        let mut file = File::create("test-input.txt").unwrap();
        file.write_all(&input.as_bytes()).unwrap();
        drop(file);

        get_lines("test-input.txt")
    }
}