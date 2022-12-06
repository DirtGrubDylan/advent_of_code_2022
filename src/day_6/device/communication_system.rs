use std::collections::VecDeque;

pub const START_OF_PACKET_MARKER_SIZE: usize = 4;
pub const START_OF_MESSAGE_MARKER_SIZE: usize = 14;

pub fn start_of_packet_marker_index(data_stream_buffer: &str, data_start_index: usize) -> usize {
    start_of_n_unique_chars_index(
        data_stream_buffer,
        data_start_index,
        START_OF_PACKET_MARKER_SIZE,
    )
}

pub fn start_of_message_marker_index(data_stream_buffer: &str, data_start_index: usize) -> usize {
    start_of_n_unique_chars_index(
        data_stream_buffer,
        data_start_index,
        START_OF_MESSAGE_MARKER_SIZE,
    )
}

fn start_of_n_unique_chars_index(data: &str, data_start_index: usize, n: usize) -> usize {
    let mut result_index = 0;
    let mut number_of_duplicates = 0;
    let mut window: VecDeque<char> = VecDeque::new();

    for (index, value) in data.char_indices().skip(data_start_index) {
        result_index = index;

        if window.len() == n {
            number_of_duplicates -= window
                .pop_front()
                .map_or(0, |pop| window.contains(&pop) as usize);
        }

        number_of_duplicates += window.contains(&value) as usize;

        window.push_back(value);

        if (window.len() == n) && (number_of_duplicates == 0) {
            break;
        }
    }

    result_index + 1 - n
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
            .map(|line| start_of_packet_marker_index(line, 0))
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
            .map(|line| start_of_message_marker_index(line, 0))
            .collect();

        assert_eq!(result, expected);
    }
}
