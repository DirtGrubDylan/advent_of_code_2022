mod forest;

use crate::util::file_reader::to_string_vector;

use forest::Forest;

pub fn run() {
    let input = to_string_vector("inputs/day_8.txt").expect("Something went wrong Day 8!");

    let forest = Forest::from(&input);

    println!("Day 8, Part 1: {}", part_1(&forest));
    println!("Day 8, Part 2: {:?}", part_2(&forest));
}

fn part_1(forest: &Forest) -> usize {
    forest
        .tree_visibility()
        .iter()
        .filter(|(_, direction_visible)| !direction_visible.is_empty())
        .count()
}

fn part_2(forest: &Forest) -> Option<usize> {
    forest.tree_scenic_scores().values().max().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = 21;

        let result = part_1(&forest);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = Some(8);

        let result = part_2(&forest);

        assert_eq!(result, expected);
    }
}
