mod rope;

use crate::util::file_reader::to_string_vector;
use rope::{Motion, Rope};

pub fn run() {
    let input = to_string_vector("inputs/day_9.txt").expect("Something went wrong Day 9!");

    let mut rope_2 = Rope::new(2);
    let mut rope_10 = Rope::new(10);

    let motions: Vec<Motion> = input.iter().map(|line| Motion::from(line)).collect();

    let part_1 = rope_2.apply_motions(&motions).len();
    let part_2 = rope_10.apply_motions(&motions).len();

    println!("Day 9, Part 1: {}", part_1);
    println!("Day 9, Part 2: {}", part_2);
}
