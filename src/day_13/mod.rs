mod distress;

use std::cmp::Ordering;

use crate::util::file_reader::to_string_vector;

use distress::{Packet, PacketOrder};

pub fn run() {
    let input = to_string_vector("inputs/day_13.txt").expect("Something went wrong Day 13!");

    let packets = get_packet_pairs(&input);

    println!("Day 13, Part 1: {}", part_1(&packets));
    println!("Day 13, Part 2: {}", part_2(&packets));
}

fn part_1(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .map(|(index, (first, second))| (index + 1, first.get_order(second)))
        .filter(|(_, order)| *order == PacketOrder::In)
        .fold(0, |acc, (index, _)| acc + index)
}

fn part_2(packets: &[(Packet, Packet)]) -> usize {
    let divider_packet_1 = Packet::from(vec!['[', '[', '6', ']', ']'].as_slice());
    let divider_packet_2 = Packet::from(vec!['[', '[', '2', ']', ']'].as_slice());

    let mut result: Vec<&Packet> = packets
        .iter()
        .map(|(first, other)| match first.get_order(&other) {
            PacketOrder::In => vec![first, other],
            PacketOrder::Out => vec![other, first],
            PacketOrder::Unknown => panic!(),
        })
        .flatten()
        .collect();

    result.push(&divider_packet_1);
    result.push(&divider_packet_2);

    result.sort_by(|packet, other| match packet.get_order(other) {
        PacketOrder::In => Ordering::Less,
        PacketOrder::Out => Ordering::Greater,
        PacketOrder::Unknown => Ordering::Equal,
    });

    result
        .iter()
        .enumerate()
        .filter(|(_, &packet)| packet == &divider_packet_1 || packet == &divider_packet_2)
        .map(|(index, _)| index + 1)
        .fold(1, |acc, index| acc * index)
}

fn get_packet_pairs(input: &[String]) -> Vec<(Packet, Packet)> {
    input
        .split(|line| line.is_empty())
        .map(|split| get_pair(split))
        .collect()
}

fn get_pair(input: &[String]) -> (Packet, Packet) {
    let left_chars: Vec<char> = input[0].chars().collect();
    let right_chars: Vec<char> = input[1].chars().collect();

    let left = Packet::from(left_chars.as_slice());
    let right = Packet::from(right_chars.as_slice());

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_13_part_1.txt").unwrap();

        let packets = get_packet_pairs(&input);

        let expected = 13;

        let result = part_1(&packets);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_13_part_1.txt").unwrap();

        let packets = get_packet_pairs(&input);

        let expected = 140;

        let result = part_2(&packets);

        assert_eq!(result, expected);
    }
}
