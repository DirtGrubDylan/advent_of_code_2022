mod elf;

use crate::util::file_reader::to_string_vector;

use elf::Elf;

pub fn run() {
    let input = to_string_vector("inputs/day_1.txt").expect("Something went wrong with Day 1!");

    let mut elves = get_elves(&input);

    elves.sort_by(|x, y| {
        y.total_calories_from_rations()
            .cmp(&x.total_calories_from_rations())
    });

    let top_3_elves: Vec<Elf> = elves.into_iter().take(3).collect();

    let elf_carrying_most_calories = top_3_elves.get(0);

    println!(
        "Day 1, Part 1: {}",
        elf_carrying_most_calories.map_or(0, |elf| elf.total_calories_from_rations())
    );
    println!(
        "Day 1, Part 2: {}",
        top_3_elves
            .iter()
            .fold(0, |acc, elf| acc + elf.total_calories_from_rations())
    );
}

fn get_elves(input: &[String]) -> Vec<Elf> {
    let split_input = input.split(|calories| calories.is_empty());

    split_input.map(|split| Elf::new(split)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALORIES: [&'static str; 14] = [
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn test_get_elves() {
        let input: Vec<String> = CALORIES.iter().map(|s| s.to_string()).collect();

        let expected = vec![
            Elf::new(&[
                String::from("1000"),
                String::from("2000"),
                String::from("3000"),
            ]),
            Elf::new(&[String::from("4000")]),
            Elf::new(&[String::from("5000"), String::from("6000")]),
            Elf::new(&[
                String::from("7000"),
                String::from("8000"),
                String::from("9000"),
            ]),
            Elf::new(&[String::from("10000")]),
        ];

        let result = get_elves(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_elf_carrying_most_calories() {
        let input: Vec<String> = CALORIES.iter().map(|s| s.to_string()).collect();

        let mut elves = get_elves(&input);

        elves.sort_by(|x, y| {
            y.total_calories_from_rations()
                .cmp(&x.total_calories_from_rations())
        });

        let expected_elf = Some(Elf::new(&[
            String::from("7000"),
            String::from("8000"),
            String::from("9000"),
        ]));
        let expected_calories = 24_000;

        let result_elf = elves.get(0).cloned();
        let result_calories = result_elf
            .as_ref()
            .map_or(0, |elf| elf.total_calories_from_rations());

        assert_eq!(result_elf, expected_elf);
        assert_eq!(result_calories, expected_calories);
    }

    #[test]
    fn test_get_top_3_elves_carrying_most_calories() {
        let input: Vec<String> = CALORIES.iter().map(|s| s.to_string()).collect();

        let mut elves = get_elves(&input);

        elves.sort_by(|x, y| {
            y.total_calories_from_rations()
                .cmp(&x.total_calories_from_rations())
        });

        let expected_top_elves = vec![
            Elf::new(&[
                String::from("7000"),
                String::from("8000"),
                String::from("9000"),
            ]),
            Elf::new(&[String::from("5000"), String::from("6000")]),
            Elf::new(&[String::from("10000")]),
        ];
        let expected_calories = 45_000;

        let result_top_elves: Vec<Elf> = elves.into_iter().take(3).collect();
        let result_calories = result_top_elves
            .iter()
            .fold(0, |acc, elf| acc + elf.total_calories_from_rations());

        assert_eq!(result_top_elves, expected_top_elves);
        assert_eq!(result_calories, expected_calories);
    }
}
