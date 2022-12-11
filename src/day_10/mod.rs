use std::collections::HashSet;

use crate::device::Device;
use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_10.txt").expect("Something went wrong Day 10!");

    let mut device = Device::new();

    device.add_video_system_cpu_instructions(&input);

    println!("Day 10, Part 1: {}", part_1(&mut device));
}

fn part_1(device: &mut Device) -> i32 {
    device
        .get_video_system_x_signal_strengths_at(&HashSet::from([20, 60, 100, 140, 180, 220]))
        .into_iter()
        .fold(0, |acc, signal_strength| acc + signal_strength)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::file_reader::to_string_vector;

    #[test]
    fn test() {
        let input = to_string_vector("test_inputs/day_10_part_1.txt").unwrap();

        let mut device = Device::new();

        device.add_video_system_cpu_instructions(&input);

        let expected = 13_140;

        let result = part_1(&mut device);

        assert_eq!(result, expected);
    }
}
