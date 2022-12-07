use crate::{ line_manager, types::Lines };

mod types;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> i32 {
    todo!()
}

fn problem2(lines: Lines) -> i32 {
    todo!()
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