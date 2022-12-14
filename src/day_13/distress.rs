use std::vec;

#[derive(Debug, PartialEq)]
enum Packet {
    Value(usize),
    List(Vec<Box<Packet>>),
}

impl Packet {
    fn get_size(&self) -> usize {
        unimplemented!()
    }
}

impl From<&[char]> for Packet {
    fn from(input: &[char]) -> Packet {
        let mut current_index = 0;
        let mut current_value = 0;
        let mut sub_packets: Vec<Box<Packet>> = vec![];

        let mut current_packet: Option<Packet> = None;

        while current_index < input.len() {
            let current_char = input[current_index];

            match current_char {
                ('0'..='9') => {
                    current_packet = Some(Packet::Value(0));

                    current_value *= 10;
                    current_value += current_char.to_digit(10).unwrap();
                }
                '[' => {
                    current_packet = Some(Packet::List(vec![]));

                    let temp = Box::new(Packet::from(&input[(current_index + 1)..]));

                    sub_packets.push(temp);
                }
                ',' => match current_packet {
                    Some(Packet::Value(_)) => break,
                    Some(Packet::List(_)) => {
                        let temp = Box::new(Packet::from(&input[(current_index + 1)..]));

                        sub_packets.push(temp);
                    }
                    None => panic!(),
                },
                ']' => {
                    break;
                }
                _ => panic!(),
            }

            current_index += 1;
        }

        match current_packet {
            Some(Packet::Value(_)) => Packet::Value(current_value as usize),
            Some(Packet::List(_)) => Packet::List(sub_packets),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_value_size() {
        let packet_1 = Packet::Value(1);
        let packet_2 = Packet::Value(10);
        let packet_3 = Packet::Value(110);

        let expected_1 = 1;
        let expected_2 = 2;
        let expected_3 = 3;

        let result_1 = packet_1.get_size();
        let result_2 = packet_2.get_size();
        let result_3 = packet_3.get_size();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }

    #[test]
    fn test_packet_list_of_values_size() {
        // 01234567890123
        //"[1,110,30,1,1]"
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(110)),
            Box::new(Packet::Value(30)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);

        // 5 items: 8 chars && 4 commas
        let expected = 12;

        let result = packet.get_size();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_value_from() {
        let input: Vec<char> = "[1,10,3,1,1]".chars().collect();

        let expected = Packet::Value(10);

        let result = Packet::from(&input[3..]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_list_of_values_from() {
        let input: Vec<char> = "[1,110,30,1,1]".chars().collect();

        let expected = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(110)),
            Box::new(Packet::Value(30)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);

        let result = Packet::from(&input[0..]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_list_of_lists_from() {
        let input: Vec<char> = "[[1,110,30],[1,1], []]".chars().collect();

        let expected = Packet::List(vec![
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(1)),
                Box::new(Packet::Value(110)),
                Box::new(Packet::Value(30)),
            ])),
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(1)),
                Box::new(Packet::Value(1)),
            ])),
            Box::new(Packet::List(vec![])),
        ]);

        let result = Packet::from(&input[0..]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_list_of_lists_and_values_from() {
        let input: Vec<char> = "[[1,110,30], 1, 1, []]".chars().collect();

        let expected = Packet::List(vec![
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(1)),
                Box::new(Packet::Value(110)),
                Box::new(Packet::Value(30)),
            ])),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::List(vec![])),
        ]);

        let result = Packet::from(&input[0..]);

        assert_eq!(result, expected);
    }
}
