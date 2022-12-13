mod hill_climber;

use crate::util::file_reader::to_string_vector;

use hill_climber::HeightMap;

pub fn run() {
    let input = to_string_vector("inputs/day_12.txt").expect("Something went wrong Day 12!");

    let height_map = HeightMap::from(&input);

    let shortest_path = height_map.get_shortest_path();

    println!("Day 12, Part 1: {:?}, {:?}", height_map.start, height_map.end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let height_map = HeightMap::from(&input);

        let expected = 31;

        let result = height_map.get_shortest_path().len() - 1;

        assert_eq!(result, expected);
    }
}
