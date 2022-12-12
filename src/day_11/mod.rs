mod monkey;

use std::collections::HashMap;

use crate::util::file_reader::to_string_vector;

use monkey::Monkey;

pub fn run() {
    let input = to_string_vector("inputs/day_11.txt").expect("Something went wrong Day 11!");

    println!("Day 11, Part 1: {}", part_1(&input));
}

fn part_1(input: &[String]) -> usize {
    let mut monkeys: HashMap<usize, Monkey> = input
        .split(|line| line.is_empty())
        .map(|props| Monkey::from(props))
        .map(|monkey| (monkey.id, monkey))
        .collect();

    let number_of_monkeys = monkeys.len();

    let total_passes = 20 * number_of_monkeys;

    for pass in 0..total_passes {
        let monkey_id = pass % number_of_monkeys;

        let current_monkey = monkeys.get_mut(&monkey_id).unwrap();

        for (next_monkey_id, worry_value) in current_monkey.take_turn(|worry_level| worry_level / 3)
        {
            let next_monkey = monkeys.get_mut(&next_monkey_id).unwrap();

            next_monkey.take_item(worry_value);
        }
    }

    let mut monkey_number_items_inspected: Vec<usize> = monkeys
        .into_iter()
        .map(|(_, monkey)| monkey.number_of_items_inspected)
        .collect();

    monkey_number_items_inspected.sort_by(|a, b| b.cmp(a));

    monkey_number_items_inspected[0] * monkey_number_items_inspected[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("Monkey 0:"),
            String::from("  Starting items: 79, 98"),
            String::from("  Operation: new = old * 19"),
            String::from("  Test: divisible by 23"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 3"),
            String::from(""),
            String::from("Monkey 1:"),
            String::from("  Starting items: 54, 65, 75, 74"),
            String::from("  Operation: new = old + 6"),
            String::from("  Test: divisible by 19"),
            String::from("    If true: throw to monkey 2"),
            String::from("    If false: throw to monkey 0"),
            String::from(""),
            String::from("Monkey 2:"),
            String::from("  Starting items: 79, 60, 97"),
            String::from("  Operation: new = old * old"),
            String::from("  Test: divisible by 13"),
            String::from("    If true: throw to monkey 1"),
            String::from("    If false: throw to monkey 3"),
            String::from(""),
            String::from("Monkey 3:"),
            String::from("  Starting items: 74"),
            String::from("  Operation: new = old + 3"),
            String::from("  Test: divisible by 17"),
            String::from("    If true: throw to monkey 0"),
            String::from("    If false: throw to monkey 1"),
        ];

        let expected = 10_605;

        let result = part_1(&input);

        assert_eq!(result, expected);
    }
}
