mod rope;

use crate::util::file_reader::to_string_vector;
use rope::{Motion, Rope};

pub fn run() {
    let input = to_string_vector("inputs/day_9.txt").expect("Something went wrong Day 9!");

    let mut rope = Rope::new(2);

    let motions: Vec<Motion> = input.iter().map(|line| Motion::from(line)).collect();

    let part_1 = part_1(&mut rope, &motions);

    println!("Day 9, Part 1: {}", part_1);
}

fn part_1(rope: &mut Rope, motions: &[Motion]) -> usize {
    rope.apply_motions(motions).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let motions = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];

        let mut rope = Rope::new(2);

        let expected = 13;

        let result = part_1(&mut rope, &motions);

        assert_eq!(result, expected);
    }
}
