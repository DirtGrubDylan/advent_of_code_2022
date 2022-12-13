use std::collections::HashMap;

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
pub struct HeightMap {
    start: Point2d<i32>,
    end: Point2d<i32>,
    points: HashMap<Point2d<i32>, char>,
}

impl From<&Vec<String>> for HeightMap {
    fn from(input: &Vec<String>) -> HeightMap {
        unimplemented!()
    }
}

impl HeightMap {
    fn new() -> HeightMap {
        HeightMap {
            start: Point2d::new(0, 0),
            end: Point2d::new(0, 0),
            points: HashMap::new(),
        }
    }

    fn get_height_at(&self, point: &Point2d<i32>) -> char {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let expected = HeightMap::new();

        let result = HeightMap::from(&input);

        assert_eq!(result, expected);
    }
}
