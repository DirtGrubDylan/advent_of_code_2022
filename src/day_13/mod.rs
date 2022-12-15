mod distress;

use crate::util::file_reader::to_string_vector;

use distress::{Packet, PacketOrder};

pub fn run() {
    let input = to_string_vector("inputs/day_13.txt").expect("Something went wrong Day 13!");

    unimplemented!()
}

fn pair_in_order(left_str: &str, right_str: &str) -> bool {
    let left_chars: Vec<char> = left_str.chars().collect();
    let right_chars: Vec<char> = left_str.chars().collect();

    let left = Packet::from(left_chars.as_slice());
    let right = Packet::from(right_chars.as_slice());

    match left.get_order(&right) {
        PacketOrder::In => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unimplemented!()
    }
}
