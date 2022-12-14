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
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_value_from() {
        let input: Vec<char> = "[1,10,3,1,1]".chars().collect();

        let expected = Packet::Value(10);

        let result = Packet::from(&input[4..]);

        assert_eq!(result, expected);
    }
}
