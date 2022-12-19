mod cave;

use crate::util::file_reader::to_string_vector;

use cave::Cave;

pub fn run() {
    let input = to_string_vector("inputs/day_14.txt").expect("Something went wrong Day 14!");

    let mut cave = Cave::new();

    input.iter().for_each(|line| {
        cave.add_rock(line);
    });

    println!("Day 14, Part 1: {}", part_1(&mut cave));
}

fn part_1(cave: &mut Cave) -> usize {
    let mut number_of_grains = 0;

    while cave.drop_sand().is_some() {
        number_of_grains += 1;
    }

    number_of_grains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let mut cave = Cave::new();

        input.iter().for_each(|line| cave.add_rock(&line));

        let expected = 24;

        let result = part_1(&mut cave);

        assert_eq!(result, expected);
    }
}
