use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Monkey {
    id: usize,
    items: VecDeque<i32>,
    number_of_items_inspected: usize,
    operation: char,
    operation_scalar: Option<i32>,
    test_scalar: i32,
    test_true_monkey_id: usize,
    test_false_monkey_id: usize,
}

impl From<&Vec<String>> for Monkey {
    fn from(input: &Vec<String>) -> Monkey {
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

        let result = Monkey::from(&input);

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

        let result = Monkey::from(&input);

        assert_eq!(result, expected);
    }
}
