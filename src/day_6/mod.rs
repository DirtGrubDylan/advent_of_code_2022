use crate::device::Device;
use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_6.txt").expect("Something went wrong Day 6!");

    let device = input
        .into_iter()
        .map(|line| Device::from(&line))
        .nth(0)
        .expect("Could not build Device!");

    let number_of_characters_before_first_start_of_packet = device
        .number_of_characters_before_first_start_of_packet()
        .expect("Couldn't find packet marker.");
    let number_of_characters_before_first_start_of_message = device
        .number_of_characters_before_first_start_of_message()
        .expect("Couldn't find message marker.");

    println!(
        "Day 6, Part 1: {}",
        number_of_characters_before_first_start_of_packet
    );
    println!(
        "Day 6, Part 2: {}",
        number_of_characters_before_first_start_of_message
    );
}
