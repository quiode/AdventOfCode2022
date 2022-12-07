use std::{ str::FromStr, convert::Infallible, fs, io::Write };

use crate::{ line_manager, types::Lines };

use self::types::{ File, NavOption };

mod types;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> u64 {
    let directory = build_directory(lines);

    let mut file = fs::File::create("test.json").unwrap();

    file.write_all(serde_json::to_string_pretty(&directory).unwrap().as_bytes()).unwrap();

    directory.smallest_dirs_total(100_000)
}

fn problem2(lines: Lines) -> i32 {
    todo!()
}

fn build_directory(lines: Lines) -> File {
    let main_dir = File::new(types::FileType::Directory, 0, "/");
    let mut zipper = main_dir.zipper();

    for line in lines {
        let line = line.unwrap();
        let instruction = Instruction::from_str(&line).unwrap();

        match instruction {
            Instruction::NavInstruction(nav_option) => {
                zipper = zipper.cd(nav_option);
            }
            Instruction::File(file) => {
                zipper.borrow_file_mut().add_child(file);
            }
            Instruction::None => {}
        }
    }

    zipper.finish()
}

enum Instruction {
    NavInstruction(NavOption),
    File(File),
    None,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(" cd ") {
            let mut info = s.split(" ");
            let path = info.nth(2).unwrap();
            Ok(Instruction::NavInstruction(NavOption::from_str(path).unwrap()))
        } else {
            if s.contains("dir ") {
                let mut info = s.split(" ");
                Ok(
                    Instruction::File(
                        File::new(types::FileType::Directory, 0, info.nth(1).unwrap())
                    )
                )
            } else if s.contains(" ls") {
                Ok(Instruction::None)
            } else {
                let mut info = s.split(" ");
                Ok(
                    Instruction::File(
                        File::new(
                            types::FileType::File,
                            info.next().unwrap().parse().unwrap(),
                            info.next().unwrap()
                        )
                    )
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::problem1;
    use crate::line_manager::{ create_lines, TEST_FILE };

    #[test]
    fn problem1_test() {
        let result = problem1(
            create_lines(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
",
                TEST_FILE
            )
        );

        assert_eq!(result, 95437);
    }

    #[test]
    fn problem2_test() {
        todo!()
    }
}