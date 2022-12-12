use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Monkey {
    pub id: usize,
    items: VecDeque<i32>,
    pub number_of_items_inspected: usize,
    operation: char,
    operation_scalar: Option<i32>,
    test_scalar: i32,
    test_true_monkey_id: usize,
    test_false_monkey_id: usize,
}

impl Monkey {
    pub fn take_turn(&mut self, worry_function: fn(i32) -> i32) -> Vec<(usize, i32)> {
        let mut result = vec![];

        while let Some(mut worry_level) = self.items.pop_front() {
            self.number_of_items_inspected += 1;

            worry_level = self.apply_operation(worry_level);

            worry_level = worry_function(worry_level);

            let to_monkey = self.get_monkey_to_throw_to(worry_level);

            result.push((to_monkey, worry_level));
        }

        result
    }

    pub fn take_item(&mut self, worry_value: i32) {
        self.items.push_back(worry_value);
    }

    pub fn must_divide_by(&self) -> i32 {
        self.test_scalar
    }

    fn apply_operation(&self, worry_level: i32) -> i32 {
        let scalar = self.operation_scalar.unwrap_or(worry_level);

        match &self.operation {
            '+' => worry_level + scalar,
            '-' => worry_level - scalar,
            '*' => worry_level * scalar,
            '/' => worry_level / scalar,
            _ => panic!("Unknown operation: {}", self.operation),
        }
    }

    fn get_monkey_to_throw_to(&self, worry_level: i32) -> usize {
        if self.run_test(worry_level) {
            self.test_true_monkey_id
        } else {
            self.test_false_monkey_id
        }
    }

    fn run_test(&self, worry_level: i32) -> bool {
        worry_level % self.test_scalar == 0
    }
}

impl From<&[String]> for Monkey {
    fn from(input: &[String]) -> Monkey {
        let id = get_only_digit(&input[0]).unwrap();

        let items = input[1][18..]
            .split(", ")
            .filter_map(|item| item.parse().ok())
            .collect();

        let number_of_items_inspected = 0;

        let operation_info = input[2][23..].split_once(' ').unwrap();

        let operation = operation_info.0.chars().nth(0).unwrap();

        let operation_scalar = operation_info.1.parse().ok();

        let test_scalar = get_only_digit(&input[3]).unwrap();
        let test_true_monkey_id = get_only_digit(&input[4]).unwrap();
        let test_false_monkey_id = get_only_digit(&input[5]).unwrap();

        Monkey {
            id,
            items,
            number_of_items_inspected,
            operation,
            operation_scalar,
            test_scalar,
            test_true_monkey_id,
            test_false_monkey_id,
        }
    }
}

fn get_only_digit<T>(input: &str) -> Option<T>
where
    T: FromStr<Err = ParseIntError>,
{
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_from() {
        let input = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old * 19"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
        ];

        let expected = Monkey {
            id: 0,
            items: VecDeque::from([79, 98]),
            number_of_items_inspected: 0,
            operation: '*',
            operation_scalar: Some(19),
            test_scalar: 23,
            test_true_monkey_id: 2,
            test_false_monkey_id: 3,
        };

        let result = Monkey::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_monkey_from_none_operation_scalar() {
        let input = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old + old"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
        ];

        let expected = Monkey {
            id: 0,
            items: VecDeque::from([79, 98]),
            number_of_items_inspected: 0,
            operation: '+',
            operation_scalar: None,
            test_scalar: 23,
            test_true_monkey_id: 2,
            test_false_monkey_id: 3,
        };

        let result = Monkey::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_money_take_turn_all_true() {
        let input = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 69"),
            String::from("  Operation: new = old + old"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
        ];

        let mut monkey = Monkey::from(input.as_slice());

        let expected = vec![(2, 46)];

        let result = monkey.take_turn(|worry_level| worry_level / 3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_money_take_turn_all_false() {
        let input = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old * 19"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
        ];

        let mut monkey = Monkey::from(input.as_slice());

        let expected = vec![(3, 500), (3, 620)];

        let result = monkey.take_turn(|worry_level| worry_level / 3);

        assert_eq!(result, expected);
    }
}
