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

mod imp {
    struct Monkey {
        // items of the monkey (i32 is stress level)
        items: Vec<i32>,
        // opperation the monkey performs
        operation: (OperationValue, Opperand, OperationValue),
        // test to test to which monkey it throws next
        test: TestType,
        // number of inspections
        inspections: usize,
    }

    enum OperationValue {
        Number(i32),
        Old,
    }

    enum Opperand {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    struct Test {
        // test case to test
        test: TestType,
        // monkey id if test is true
        if_true: usize,
        // monkey id if test if false
        if_false: usize,
    }

    enum TestType {
        Divisible(i32),
    }

    // function to execute on the worry_level after the monkey performs the opperation
    fn relax(worry_level: i32) -> i32 {
        worry_level / 3
    }
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