mod communication_system;

use communication_system::{START_OF_MESSAGE_MARKER_SIZE, START_OF_PACKET_MARKER_SIZE};

#[derive(Debug, PartialEq)]
pub struct Device {
    data_stream_buffer: String,
}

impl Device {
    pub fn number_of_characters_before_first_start_of_packet(&self) -> usize {
        communication_system::start_of_packet_marker_index(&self.data_stream_buffer, 0)
            + START_OF_PACKET_MARKER_SIZE
    }

    pub fn number_of_characters_before_first_start_of_message(&self) -> usize {
        communication_system::start_of_message_marker_index(&self.data_stream_buffer, 0)
            + START_OF_MESSAGE_MARKER_SIZE
    }
}

impl From<&String> for Device {
    fn from(input: &String) -> Device {
        Device {
            data_stream_buffer: input.clone(),
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
            .map(|device| device.number_of_characters_before_first_start_of_packet())
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
            .map(|device| device.number_of_characters_before_first_start_of_message())
            .collect();

        assert_eq!(result, expected);
    }
}
