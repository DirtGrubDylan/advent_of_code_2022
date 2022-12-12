#[derive(Debug, PartialEq)]
struct Monkey {}

impl From<&Vec<String>> for Monkey {
    fn from(input: &Vec<String>) -> Monkey {
        unimplemented!()
    }
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

        let expected = Monkey {};

        let result = Monkey::from(&input);

        assert_eq!(result, expected);
    }
}
