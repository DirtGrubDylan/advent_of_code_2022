mod supplies;

use crate::util::file_reader::to_string_vector;

use supplies::{Crane, Instruction};

pub fn run() {
    let input = to_string_vector("inputs/day_5.txt").expect("Something went wrong Day 5!");

    let split_input: Vec<Vec<String>>  = input.split(|line| line.is_empty()).map(|split| split.into()).collect();

    let mut crane_9000 = Crane::from(&split_input[0]);
    let mut crane_9001 = Crane::from(&split_input[0]);

    let instructions: Vec<Instruction> = split_input[1].iter().map(|line| Instruction::from(line)).collect();

    instructions.iter().for_each(|instruction| crane_9000.execute_9000(instruction));
    instructions.iter().for_each(|instruction| crane_9001.execute_9001(instruction));

    let top_supplies_9000 = crane_9000.top_supplies();
    let top_supplies_9001 = crane_9001.top_supplies();

    println!("Day 5, Part 1: {}", top_supplies_9000.iter().collect::<String>());
    println!("Day 5, Part 2: {}", top_supplies_9001.iter().collect::<String>());
}

