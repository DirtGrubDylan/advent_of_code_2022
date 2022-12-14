use std::vec;

#[derive(Debug, PartialEq)]
enum Packet {
    Value(usize),
    List(Vec<Box<Packet>>),
}

impl Packet {
    fn new_value() -> Packet {
        Packet::Value(0)
    }

    fn new_list() -> Packet {
        Packet::List(vec![])
    }

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
                    current_packet = Some(Packet::new_value());

                    current_value *= 10;
                    current_value += current_char.to_digit(10).unwrap();
                }
                '[' => {
                    current_packet = Some(Packet::new_value());

                    let sub_packet = Box::new(Packet::from(&input[(current_index + 1)..]));

                    sub_packets.push(sub_packet);
                }
                ',' => {
                    break;
                }
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
