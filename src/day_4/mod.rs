mod assignments;

use crate::util::file_reader::to_string_vector;

use assignments::SectionAssignments;

type SectionAssignmentPairs = (SectionAssignments, SectionAssignments);

pub fn run() {
    let input = to_string_vector("inputs/day_4.txt").expect("Something went wrong with Day 4!");

    let section_assignment_pairs: Vec<SectionAssignmentPairs> = input
        .iter()
        .map(|line| get_section_assignment_pairs(line))
        .collect();

    let number_of_pairs_with_strict_subsets = section_assignment_pairs
        .iter()
        .filter(|pair| pair_has_strict_subset(pair))
        .count();

    let number_of_non_disjoint_pairs = section_assignment_pairs
        .iter()
        .filter(|pair| !pair_is_disjoint(pair))
        .count();

    println!("Day 4, Part 1: {}", number_of_pairs_with_strict_subsets);
    println!("Day 4, Part 2: {}", number_of_non_disjoint_pairs);
}

fn get_section_assignment_pairs(input: &str) -> SectionAssignmentPairs {
    input
        .split_once(',')
        .map(|(first, second)| {
            (
                SectionAssignments::from(first),
                SectionAssignments::from(second),
            )
        })
        .expect(&format!("Could not parse: {}", input))
}

fn pair_has_strict_subset(pair: &SectionAssignmentPairs) -> bool {
    pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
}

fn pair_is_disjoint(pair: &SectionAssignmentPairs) -> bool {
    !pair.0.overlaps(&pair.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_section_assignment_pairs() {
        let input = "6-6,4-6";

        let expected = (
            SectionAssignments::from("6-6"),
            SectionAssignments::from("4-6"),
        );

        let result = get_section_assignment_pairs(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pair_has_strict_subset() {
        let input = vec![
            (
                SectionAssignments::from("2-4"),
                SectionAssignments::from("6-8"),
            ),
            (
                SectionAssignments::from("2-3"),
                SectionAssignments::from("4-5"),
            ),
            (
                SectionAssignments::from("5-7"),
                SectionAssignments::from("7-9"),
            ),
            (
                SectionAssignments::from("2-8"),
                SectionAssignments::from("3-7"),
            ),
            (
                SectionAssignments::from("6-6"),
                SectionAssignments::from("4-6"),
            ),
            (
                SectionAssignments::from("2-6"),
                SectionAssignments::from("4-8"),
            ),
        ];

        let expected = vec![false, false, false, true, true, false];

        let result: Vec<bool> = input
            .iter()
            .map(|pair| pair_has_strict_subset(pair))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pair_is_disjoint() {
        let input = vec![
            (
                SectionAssignments::from("2-4"),
                SectionAssignments::from("6-8"),
            ),
            (
                SectionAssignments::from("2-3"),
                SectionAssignments::from("4-5"),
            ),
            (
                SectionAssignments::from("5-7"),
                SectionAssignments::from("7-9"),
            ),
            (
                SectionAssignments::from("2-8"),
                SectionAssignments::from("3-7"),
            ),
            (
                SectionAssignments::from("6-6"),
                SectionAssignments::from("4-6"),
            ),
            (
                SectionAssignments::from("2-6"),
                SectionAssignments::from("4-8"),
            ),
        ];

        let expected = vec![true, true, false, false, false, false];

        let result: Vec<bool> = input.iter().map(|pair| pair_is_disjoint(pair)).collect();

        assert_eq!(result, expected);
    }
}
