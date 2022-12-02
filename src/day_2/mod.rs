mod rock_paper_scissors;

use crate::util::file_reader::to_string_vector;

use rock_paper_scissors::Round;

pub fn run() {
    let input = to_string_vector("inputs/day_2.txt").expect("Something went wrong with Day 2!");

    let rounds = get_rounds(&input);

    let total_score = get_total_score(&rounds);
    let total_expected_score = get_total_expected_score(&rounds);

    println!("Day 2, Part 1: {}", total_score);
    println!("Day 2, Part 2: {}", total_expected_score);
}

fn get_rounds(input: &[String]) -> Vec<Round> {
    input.iter().map(|s| Round::new(s)).collect()
}

fn get_total_score(rounds: &[Round]) -> u32 {
    rounds.iter().fold(0, |acc, round| acc + round.get_score())
}

fn get_total_expected_score(rounds: &[Round]) -> u32 {
    rounds
        .iter()
        .fold(0, |acc, round| acc + round.get_expected_output_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rounds() {
        let input = vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z"),
        ];

        let expected = vec![Round::new("A Y"), Round::new("B X"), Round::new("C Z")];

        let result = get_rounds(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_total_score() {
        let rounds = vec![Round::new("A Y"), Round::new("B X"), Round::new("C Z")];

        let expected = 15;

        let result = get_total_score(&rounds);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_total_expected_score() {
        let rounds = vec![Round::new("A Y"), Round::new("B X"), Round::new("C Z")];

        let expected = 12;

        let result = get_total_expected_score(&rounds);

        assert_eq!(result, expected);
    }
}
