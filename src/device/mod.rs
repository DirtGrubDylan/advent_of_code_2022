mod commands;
mod communication_system;
mod file_system;
mod video_system;

use std::collections::HashSet;

use commands::ExecutedCommand;
use communication_system::{START_OF_MESSAGE_MARKER_SIZE, START_OF_PACKET_MARKER_SIZE};
use file_system::FileSystem;
use video_system::VideoSystem;

const TOTAL_FILE_SYSTEM_SIZE: u64 = 70_000_000;

#[derive(Debug, PartialEq)]
pub struct Device {
    data_stream_buffer: Vec<char>,
    file_system: FileSystem,
    video_system: VideoSystem,
}

impl Device {
    pub fn new() -> Device {
        Device {
            data_stream_buffer: vec![],
            file_system: FileSystem::new(),
            video_system: VideoSystem::new(),
        }
    }

    pub fn number_of_characters_before_first_start_of_packet(&self) -> Option<usize> {
        communication_system::start_of_packet_marker_index(&self.data_stream_buffer, 0)
            .map(|index| index + START_OF_PACKET_MARKER_SIZE)
    }

    pub fn number_of_characters_before_first_start_of_message(&self) -> Option<usize> {
        communication_system::start_of_message_marker_index(&self.data_stream_buffer, 0)
            .map(|index| index + START_OF_MESSAGE_MARKER_SIZE)
    }

    pub fn populate_file_system_from_executed_commands(&mut self, commands: &[String]) {
        self.file_system =
            FileSystem::create_from_executed_commands(&ExecutedCommand::extract_commands(commands));
    }

    pub fn available_disk_space(&self) -> u64 {
        TOTAL_FILE_SYSTEM_SIZE - self.file_system.get_size()
    }

    pub fn sum_of_directory_sizes_while<P>(&self, predicate: P) -> u64
    where
        P: Fn(u64) -> bool,
    {
        self.file_system.sum_of_directory_sizes_while(predicate)
    }

    pub fn smallest_directory_size_where<P>(&self, predicate: P) -> Option<u64>
    where
        P: Fn(u64) -> bool,
    {
        self.file_system
            .directory_sizes_while(predicate)
            .values()
            .min()
            .cloned()
    }

    pub fn add_video_system_cpu_instructions(&mut self, input: &[String]) {
        self.video_system.add_cpu_instructions(input);
    }

    pub fn get_video_system_x_signal_strengths_at(&mut self, ticks: &HashSet<usize>) -> Vec<i32> {
        self.video_system
            .get_cpu_register_signal_strengths_at('X', ticks)
            .into_iter()
            .filter_map(|value| value)
            .collect()
    }

    pub fn get_video_system_display(&self) -> Vec<Vec<char>> {
        self.video_system.get_display()
    }

    pub fn print_video_system_display(&self) {
        self.video_system.print_display();
    }
}

impl From<&String> for Device {
    fn from(input: &String) -> Device {
        Device {
            data_stream_buffer: input.chars().collect(),
            file_system: FileSystem::new(),
            video_system: VideoSystem::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_characters_before_first_start_of_packet() {
        let input = [
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ];

        let expected = vec![7, 5, 6, 10, 11];

        let result: Vec<usize> = input
            .iter()
            .map(|line| Device::from(line))
            .map(|device| {
                device
                    .number_of_characters_before_first_start_of_packet()
                    .unwrap()
            })
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_characters_before_first_start_of_message() {
        let input = [
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ];

        let expected = vec![19, 23, 23, 29, 26];

        let result: Vec<usize> = input
            .iter()
            .map(|line| Device::from(line))
            .map(|device| {
                device
                    .number_of_characters_before_first_start_of_message()
                    .unwrap()
            })
            .collect();

        assert_eq!(result, expected);
    }
}
