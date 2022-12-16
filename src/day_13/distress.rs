use std::vec;

#[derive(Debug, PartialEq)]
pub enum PacketOrder {
    Unknown,
    In,
    Out,
}

#[derive(Debug, PartialEq)]
pub enum Packet {
    Value(usize),
    List(Vec<Box<Packet>>),
}

impl Packet {
    pub fn get_order(&self, other: &Packet) -> PacketOrder {
        match (&self, &other) {
            (&Packet::Value(l), &Packet::Value(r)) if l < r => PacketOrder::In,
            (&Packet::Value(l), &Packet::Value(r)) if l == r => PacketOrder::Unknown,
            (&Packet::Value(l), &Packet::Value(r)) if r < l => PacketOrder::Out,
            (&Packet::List(_), &Packet::Value(r)) => {
                self.get_order(&Packet::List(vec![Box::new(Packet::Value(*r))]))
            }
            (&Packet::Value(l), &Packet::List(_)) => {
                Packet::List(vec![Box::new(Packet::Value(*l))]).get_order(other)
            }
            (&Packet::List(l), &Packet::List(r)) => {
                let mut result = PacketOrder::Unknown;

                for (left, right) in l.iter().zip(r.iter()) {
                    match left.get_order(right) {
                        PacketOrder::Unknown => continue,
                        PacketOrder::In => {
                            result = PacketOrder::In;

                            break;
                        }
                        PacketOrder::Out => {
                            result = PacketOrder::Out;

                            break;
                        }
                    }
                }

                match result {
                    PacketOrder::Unknown if l.len() < r.len() => PacketOrder::In,
                    PacketOrder::Unknown if r.len() < l.len() => PacketOrder::Out,
                    _ => result,
                }
            }
            _ => panic!(),
        }
    }

    fn get_size(&self) -> usize {
        match &self {
            Packet::Value(mut x) => {
                let mut result = 1;

                while x >= 10 {
                    x /= 10;

                    result += 1;
                }

                result
            }
            Packet::List(l) => {
                let number_of_commas = l.len().saturating_sub(1);
                let number_of_brackets = 2;

                let size_of_subpackets: usize = l.iter().map(|packet| packet.get_size()).sum();

                number_of_commas + number_of_brackets + size_of_subpackets
            }
        }
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

                    if input[current_index + 1] == ']' {
                        break;
                    }

                    let temp = Box::new(Packet::from(&input[(current_index + 1)..]));

                    current_index += temp.get_size();

                    sub_packets.push(temp);
                }
                ',' => match current_packet {
                    Some(Packet::Value(_)) => break,
                    Some(Packet::List(_)) => {
                        let temp = Box::new(Packet::from(&input[(current_index + 1)..]));

                        current_index += temp.get_size();

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
        let packet_4 = Packet::Value(0);

        let expected_1 = 1;
        let expected_2 = 2;
        let expected_3 = 3;
        let expected_4 = 1;

        let result_1 = packet_1.get_size();
        let result_2 = packet_2.get_size();
        let result_3 = packet_3.get_size();
        let result_4 = packet_4.get_size();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
    }

    #[test]
    fn test_packet_list_of_values_size() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(110)),
            Box::new(Packet::Value(30)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);

        let expected = 14;

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
        let input: Vec<char> = "[[1,110,30],[1,1],[]]".chars().collect();

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
        let input: Vec<char> = "[[1,110,30],1,1,[]]".chars().collect();

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

    #[test]
    fn test_packet_unknown_order_just_values() {
        let packet = Packet::Value(1);
        let other = Packet::Value(1);

        let expected = PacketOrder::Unknown;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_just_values() {
        let packet = Packet::Value(1);
        let other = Packet::Value(2);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_out_of_order_just_values() {
        let packet = Packet::Value(2);
        let other = Packet::Value(1);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_unknown_order_list_all_values() {
        let packet = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(1))]);
        let other = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(1))]);

        let expected = PacketOrder::Unknown;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_list_all_values() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(3)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(5)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_out_of_order_list_all_values() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(5)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(3)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_unknown_order_lists_same() {
        let packet = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(1))]);
        let other = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(1))]);

        let expected = PacketOrder::Unknown;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_out_of_order_list_right_runs_out() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
            Box::new(Packet::Value(1)),
        ]);
        let other = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(1))]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_list_and_value() {
        let packet = Packet::List(vec![Box::new(Packet::Value(1))]);
        let other = Packet::Value(1);

        let expected = PacketOrder::Unknown;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_list_value_and_list() {
        let packet = Packet::Value(1);
        let other = Packet::List(vec![Box::new(Packet::Value(2))]);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_out_of_order_list_and_value() {
        let packet = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(2))]);
        let other = Packet::Value(1);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_out_of_order_value_and_list() {
        let packet = Packet::Value(2);
        let other = Packet::List(vec![Box::new(Packet::Value(1)), Box::new(Packet::Value(2))]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_2() {
        let packet = Packet::List(vec![
            Box::new(Packet::List(vec![Box::new(Packet::Value(1))])),
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(2)),
                Box::new(Packet::Value(3)),
                Box::new(Packet::Value(4)),
            ])),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::List(vec![Box::new(Packet::Value(1))])),
            Box::new(Packet::Value(4)),
        ]);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_3() {
        let packet = Packet::List(vec![Box::new(Packet::Value(9))]);
        let other = Packet::List(vec![Box::new(Packet::List(vec![
            Box::new(Packet::Value(8)),
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(6)),
        ]))]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    // [[4,4],4,4]
    // [[4,4],4,4,4]
    #[test]
    fn test_packet_in_order_test_4() {
        let packet = Packet::List(vec![
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(4)),
                Box::new(Packet::Value(4)),
            ])),
            Box::new(Packet::Value(4)),
            Box::new(Packet::Value(4)),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(4)),
                Box::new(Packet::Value(4)),
            ])),
            Box::new(Packet::Value(4)),
            Box::new(Packet::Value(4)),
            Box::new(Packet::Value(4)),
        ]);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_5() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(7)),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(7)),
            Box::new(Packet::Value(7)),
        ]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_6() {
        let packet = Packet::List(vec![]);
        let other = Packet::List(vec![Box::new(Packet::Value(3))]);

        let expected = PacketOrder::In;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_7() {
        let packet = Packet::List(vec![Box::new(Packet::List(vec![Box::new(Packet::List(
            vec![],
        ))]))]);
        let other = Packet::List(vec![Box::new(Packet::List(vec![]))]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_in_order_test_8() {
        let packet = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(2)),
                Box::new(Packet::List(vec![
                    Box::new(Packet::Value(3)),
                    Box::new(Packet::List(vec![
                        Box::new(Packet::Value(4)),
                        Box::new(Packet::List(vec![
                            Box::new(Packet::Value(5)),
                            Box::new(Packet::Value(6)),
                            Box::new(Packet::Value(7)),
                        ])),
                    ])),
                ])),
            ])),
            Box::new(Packet::Value(8)),
            Box::new(Packet::Value(9)),
        ]);
        let other = Packet::List(vec![
            Box::new(Packet::Value(1)),
            Box::new(Packet::List(vec![
                Box::new(Packet::Value(2)),
                Box::new(Packet::List(vec![
                    Box::new(Packet::Value(3)),
                    Box::new(Packet::List(vec![
                        Box::new(Packet::Value(4)),
                        Box::new(Packet::List(vec![
                            Box::new(Packet::Value(5)),
                            Box::new(Packet::Value(6)),
                            Box::new(Packet::Value(0)),
                        ])),
                    ])),
                ])),
            ])),
            Box::new(Packet::Value(8)),
            Box::new(Packet::Value(9)),
        ]);

        let expected = PacketOrder::Out;

        let result = packet.get_order(&other);

        assert_eq!(result, expected);
    }
}
