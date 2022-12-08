use crate::util::file_reader::to_string_vector;

use crate::device::Device;

pub fn run() {
    let input = to_string_vector("inputs/day_7.txt").expect("Something went wrong Day 7!");

    let device = get_device(&input);

    println!("Day 7, Part 1: {}", part_1(&device));
}

fn get_device(input: &[String]) -> Device {
    let mut device = Device::new();

    device.populate_file_system_from_executed_commands(input);

    device
}

fn part_1(device: &Device) -> u64 {
    device.sum_of_directory_sizes_while(|size| size <= 100_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
            String::from("$ cd a"),
            String::from("$ ls"),
            String::from("dir e"),
            String::from("29116 f"),
            String::from("2557 g"),
            String::from("62596 h.lst"),
            String::from("$ cd e"),
            String::from("$ ls"),
            String::from("584 i"),
            String::from("$ cd .."),
            String::from("$ cd .."),
            String::from("$ cd d"),
            String::from("$ ls"),
            String::from("4060174 j"),
            String::from("8033020 d.log"),
            String::from("5626152 d.ext"),
            String::from("7214296 k"),
        ];

        let device = get_device(&input);

        let expected = 95_437;

        let result = part_1(&device);

        assert_eq!(result, expected);
    }
}
