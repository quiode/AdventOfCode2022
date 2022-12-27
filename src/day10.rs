use crate::{ types::Lines, line_manager };

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    println!("Problem 1: {}", problem1(lines));
    // println!("Problem 2: {}", problem2(lines));
}

fn problem1(lines: Lines) -> u32 {
    todo!()
}

fn problem2(lines: Lines) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem1_test() {
        todo!()
    }

    #[test]
    fn problem2_test() {
        todo!()
    }
}