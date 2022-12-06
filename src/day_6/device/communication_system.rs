use std::collections::HashSet;

pub const START_OF_PACKET_MARKER_SIZE: usize = 4;
pub const START_OF_MESSAGE_MARKER_SIZE: usize = 14;

pub fn start_of_packet_marker_index(data: &[char], data_start_index: usize) -> Option<usize> {
    start_of_n_unique_chars_index(data, data_start_index, START_OF_PACKET_MARKER_SIZE)
}

pub fn start_of_message_marker_index(data: &[char], data_start_index: usize) -> Option<usize> {
    start_of_n_unique_chars_index(data, data_start_index, START_OF_MESSAGE_MARKER_SIZE)
}

fn start_of_n_unique_chars_index(data: &[char], data_start: usize, n: usize) -> Option<usize> {
    data.windows(n)
        .enumerate()
        .skip(data_start)
        .find(|(_, window)| window.iter().cloned().collect::<HashSet<char>>().len() == window.len())
        .map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet_marker_index() {
        let input = [
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ];

        let expected = vec![3, 1, 2, 6, 7];

        let result: Vec<usize> = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .map(|line| start_of_packet_marker_index(&line, 0).unwrap())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_start_of_message_marker_index() {
        let input = [
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ];

        let expected = vec![5, 9, 9, 15, 12];

        let result: Vec<usize> = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .map(|line| start_of_message_marker_index(&line, 0).unwrap())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_start_of_n_unique_chars_index() {
        let input = [
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ];

        let expected = vec![Some(5), Some(9), Some(9), Some(15), Some(12)];

        let result: Vec<Option<usize>> = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .map(|line| start_of_n_unique_chars_index(&line, 5, 14))
            .collect();

        assert_eq!(result, expected);
    }
}
