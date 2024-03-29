use crate::{ types::Lines, line_manager };

use self::imp::*;

pub fn main() {
    let lines = line_manager::get_lines(line_manager::FILE);

    // println!("Problem 1: {}", problem1(lines));
    println!("Problem 2: {}", problem2(lines));
    // TODO: Theory for Problem 2: https://medium.com/@datasciencedisciple/advent-of-code-2022-in-python-day-11-5832b8f25c21
}

fn problem1(lines: Lines) -> u64 {
    // this works
    let mut monkeys = create_monkeys_from_lines(lines);

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            for item in monkeys[index].items.clone() {
                monkeys[index].inspections += 1;
                let mut new_worry_level = execute_operation(item, monkeys[index].operation.clone());
                new_worry_level = relax(new_worry_level);

                monkeys[index].items.remove(0);
                let new_index;
                if monkeys[index].test.execute_test(new_worry_level.clone()) {
                    new_index = monkeys[index].test.if_true;
                } else {
                    new_index = monkeys[index].test.if_false;
                }
                monkeys[new_index].items.push(new_worry_level.clone());
            }
        }
    }

    monkeys.sort_by_key(|k| k.inspections);
    return monkeys[monkeys.len() - 1].inspections * monkeys[monkeys.len() - 2].inspections;
}

fn problem2(lines: Lines) -> u64 {
    // this works
    let mut monkeys = create_monkeys_from_lines(lines);

    for _ in 0..1000 {
        for index in 0..monkeys.len() {
            for item in monkeys[index].items.clone() {
                monkeys[index].inspections += 1;
                let new_worry_level = execute_operation(item, monkeys[index].operation.clone());
                monkeys[index].items.remove(0);
                let new_index;
                if monkeys[index].test.execute_test(new_worry_level.clone()) {
                    new_index = monkeys[index].test.if_true;
                } else {
                    new_index = monkeys[index].test.if_false;
                }
                monkeys[new_index].items.push(new_worry_level.clone());
            }
        }
    }

    monkeys.sort_by_key(|k| k.inspections);
    return monkeys[monkeys.len() - 1].inspections * monkeys[monkeys.len() - 2].inspections;
}

pub mod imp {
    use std::iter::Peekable;

    use num::{ Integer, BigUint, bigint::ToBigUint };

    use crate::types::Lines;

    pub type Operation = (OperationValue, Opperand, OperationValue);

    #[derive(Debug, Clone)]
    pub struct Monkey {
        id: usize,
        // items of the monkey (i32 is stress level)
        pub items: Vec<BigUint>,
        // opperation the monkey performs
        pub operation: Operation,
        // test to test to which monkey it throws next
        pub test: Test,
        // number of inspections
        pub inspections: u64,
    }

    impl Monkey {
        fn new(
            items: Vec<BigUint>,
            operation: (OperationValue, Opperand, OperationValue),
            test: Test,
            id: usize
        ) -> Self {
            Self { id, items, operation, test, inspections: 0 }
        }

        pub fn get_id(&self) -> usize {
            self.id
        }
    }

    #[derive(Debug, Clone)]
    pub enum OperationValue {
        Number(BigUint),
        Old,
    }

    #[derive(Debug, Clone)]
    pub enum Opperand {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    #[derive(Debug, Clone)]
    pub struct Test {
        // test case to test
        test: TestType,
        // monkey id if test is true
        pub if_true: usize,
        // monkey id if test if false
        pub if_false: usize,
    }

    impl Test {
        pub fn execute_test(&self, worry_level: BigUint) -> bool {
            match &self.test {
                TestType::Divisible(num) => worry_level.is_multiple_of(&num),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum TestType {
        Divisible(BigUint),
    }

    // function to execute on the worry_level after the monkey performs the opperation
    pub fn relax(worry_level: BigUint) -> BigUint {
        worry_level / (3).to_biguint().unwrap()
    }

    pub fn execute_operation(worry_level: BigUint, operation: Operation) -> BigUint {
        let first_val;
        let second_val;

        match operation.0 {
            OperationValue::Old => {
                first_val = worry_level.clone();
            }
            OperationValue::Number(num) => {
                first_val = num;
            }
        }

        match operation.2 {
            OperationValue::Old => {
                second_val = worry_level;
            }
            OperationValue::Number(num) => {
                second_val = num;
            }
        }

        match operation.1 {
            Opperand::Add => first_val + second_val,
            Opperand::Divide => first_val / second_val,
            Opperand::Multiply => first_val * second_val,
            Opperand::Subtract => first_val - second_val,
        }
    }

    pub fn create_monkeys_from_lines(lines: Lines) -> Vec<Monkey> {
        let mut monkeys = vec![];

        let mut peekable_lines = lines.peekable();

        while peekable_lines.peek().is_some() {
            let new_monkey = create_monkey_from_lines(&mut peekable_lines, monkeys.len());
            monkeys.push(new_monkey);
        }

        monkeys
    }

    fn create_monkey_from_lines(lines: &mut Peekable<Lines>, new_id: usize) -> Monkey {
        // skip first line (monkey id)
        lines.next();

        // calculate starting items
        let mut items = vec![];
        let starting_items_string = lines.next().unwrap().unwrap();
        let index = starting_items_string.find(": ").unwrap();
        let items_string = starting_items_string.split_at(index + 2).1;
        if items_string.contains(",") {
            let items_split = items_string.split(", ");
            for item in items_split {
                items.push(item.trim().parse().unwrap());
            }
        } else {
            items.push(items_string.trim().parse().unwrap());
        }

        // calculate operation
        let operation_string = lines.next().unwrap().unwrap();
        let index = operation_string.find("new = ").unwrap();
        let operation_string = operation_string.split_at(index + 6).1.trim();
        let mut operation_strings = operation_string.split(" ");

        let next_op = operation_strings.next().unwrap();
        let first_op: OperationValue;
        if next_op == "old" {
            first_op = OperationValue::Old;
        } else {
            first_op = OperationValue::Number(next_op.parse().unwrap());
        }

        let next_op = operation_strings.next().unwrap();
        let opperand: Opperand;
        if next_op == "+" {
            opperand = Opperand::Add;
        } else if next_op == "*" {
            opperand = Opperand::Multiply;
        } else if next_op == "/" {
            opperand = Opperand::Subtract;
        } else if next_op == "-" {
            opperand = Opperand::Subtract;
        } else {
            panic!();
        }

        let next_op = operation_strings.next().unwrap();
        let second_op: OperationValue;
        if next_op == "old" {
            second_op = OperationValue::Old;
        } else {
            second_op = OperationValue::Number(next_op.parse().unwrap());
        }

        // calculate the test
        let test_case = lines.next().unwrap().unwrap();
        let index = test_case.find("by ").unwrap();
        let divisible_num = test_case
            .split_at(index + 3)
            .1.trim()
            .parse()
            .unwrap();
        let divisible_by = TestType::Divisible(divisible_num);

        // if true monkey
        let test_case = lines.next().unwrap().unwrap();
        let index = test_case.find("monkey ").unwrap();
        let monkey1_id = test_case
            .split_at(index + 7)
            .1.trim()
            .parse()
            .unwrap();
        // if false monkey
        let test_case = lines.next().unwrap().unwrap();
        let index = test_case.find("monkey ").unwrap();
        let monkey2_id = test_case
            .split_at(index + 7)
            .1.trim()
            .parse()
            .unwrap();

        let test = Test { test: divisible_by, if_true: monkey1_id, if_false: monkey2_id };

        // skip space
        lines.next();

        Monkey::new(items, (first_op, opperand, second_op), test, new_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ line_manager::{ create_lines, TEST_FILE }, day11::problem2 };

    use super::problem1;

    #[test]
    fn problem1_test() {
        let test_input =
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let lines = create_lines(test_input, TEST_FILE);
        assert_eq!(problem1(lines), 10605);
    }

    #[test]
    fn problem2_test() {
        let test_input =
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let lines = create_lines(test_input, TEST_FILE);
        assert_eq!(problem2(lines), 2713310158);
    }
}