mod hill_climber;

use crate::util::file_reader::to_string_vector;

use hill_climber::HeightMap;

pub fn run() {
    let input = to_string_vector("inputs/day_12.txt").expect("Something went wrong Day 12!");

    let height_map = HeightMap::from(&input);

    let shortest_path = height_map.get_shortest_path();
    let shortest_hiking_path = height_map.get_shortest_hiking_path();

    println!("Day 12, Part 1: {}", shortest_path.len() - 1);
    println!("Day 12, Part 2: {}", shortest_hiking_path.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
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

    #[test]
    fn test_part_2() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let height_map = HeightMap::from(&input);

        let expected = 29;

        let result = height_map.get_shortest_hiking_path().len() - 1;

        assert_eq!(result, expected);
    }
}
