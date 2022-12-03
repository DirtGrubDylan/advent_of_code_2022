mod rucksack;

use crate::util::file_reader::to_string_vector;

use rucksack::RuckSack;

pub fn run() {
    let input = to_string_vector("inputs/day_3.txt").expect("Something went wrong with Day 3!");

    let sacks: Vec<RuckSack> = input.iter().map(|line| RuckSack::new(line)).collect();

    let total_priority_sum = get_total_priority_sum(&sacks);
    let total_badge_priority_sum = get_total_badge_priority_sum(&sacks);

    println!("Day 3, Part 1: {}", total_priority_sum);
    println!("Day 3, Part 2: {}", total_badge_priority_sum);
}

fn get_total_priority_sum(input: &[RuckSack]) -> u32 {
    input.iter().fold(0, |acc, sack| {
        acc + sack.get_missorted_item_priorities().iter().sum::<u32>()
    })
}

fn get_total_badge_priority_sum(input: &[RuckSack]) -> u32 {
    input.chunks(3).fold(0, |acc, sack_group| {
        acc + RuckSack::get_shared_item_priorities(sack_group)
            .into_iter()
            .sum::<u32>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_priority_sum() {
        let input = vec![
            RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            RuckSack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            RuckSack::new("PmmdzqPrVvPwwTWBwg"),
            RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            RuckSack::new("ttgJtRGJQctTZtZT"),
            RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expected = 157;

        let result = get_total_priority_sum(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_total_badge_priority_sum() {
        let input = vec![
            RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            RuckSack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            RuckSack::new("PmmdzqPrVvPwwTWBwg"),
            RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            RuckSack::new("ttgJtRGJQctTZtZT"),
            RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expected = 70;

        let result = get_total_badge_priority_sum(&input);

        assert_eq!(result, expected);
    }
}
