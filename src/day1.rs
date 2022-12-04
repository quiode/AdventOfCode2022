use std::{ fs::File, io::{ BufRead, BufReader, Lines } };

pub fn main() {
    let input_file = File::open("input.txt").unwrap();
    let lines = BufReader::new(input_file).lines();

    println!("Biggest Cal: {}", calc_from_lines(lines));
}

fn calc_from_lines(lines: Lines<BufReader<File>>) -> i32 {
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

#[cfg(test)]
mod test {
    use std::{ fs::File, io::{ Write, self, BufRead } };

    use super::calc_from_lines;

    #[test]
    fn main() {
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

        let file = File::open("test-input.txt").unwrap();

        let result = calc_from_lines(io::BufReader::new(file).lines());

        assert_eq!(result, 24000);
    }
}