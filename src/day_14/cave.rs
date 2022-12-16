use std::collections::HashSet;

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
struct Cave {
    rocks: HashSet<Point2d<i32>>,
    sand: HashSet<Point2d<i32>>,
}

impl Cave {
    pub fn new() -> Cave {
        Cave {
            rocks: HashSet::new(),
            sand: HashSet::new(),
        }
    }

    pub fn add_rock(&mut self, rock_definition: &str) {
        unimplemented!()
    }

    pub fn add_sand_from(&mut self, starting_column: i32) {
        unimplemented!()
    }
}

fn point_from(input: &str) -> Point2d<i32> {
    input
        .split_once(',')
        .map(|(x, y)| Point2d::new(x.parse().unwrap(), y.parse().unwrap()))
        .expect(&format!("Couldn't split: {input}"))
}

fn points_to(start: Point2d<i32>, end: Point2d<i32>) -> Vec<Point2d<i32>> {
    let mut result = vec![];
    let mut current_point = start.clone();

    loop {
        result.push(current_point.clone());

        if current_point.x < end.x {
            current_point.x += 1;
        } else if end.x < current_point.x {
            current_point.x -= 1;
        }

        if current_point.y < end.y {
            current_point.y += 1;
        } else if end.y < current_point.y {
            current_point.y -= 1;
        }

        if current_point == end {
            break;
        }
    }

    result
}

fn points_along(path: &str) -> Vec<Point2d<i32>> {
    let points: Vec<Point2d<i32>> = path.split(" -> ").map(|point| point_from(point)).collect();

    let mut result: Vec<Point2d<i32>> = points
        .windows(2)
        .map(|window| points_to(window[0], window[1]))
        .flatten()
        .collect();

    result.push(points.last().unwrap().clone());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from() {
        let input = "498,4";

        let expected = Point2d::new(498, 4);

        let result = point_from(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_points_to() {
        let start = Point2d::new(0, 0);
        let end = Point2d::new(5, 5);

        let expected = vec![
            Point2d::new(0, 0),
            Point2d::new(1, 1),
            Point2d::new(2, 2),
            Point2d::new(3, 3),
            Point2d::new(4, 4),
        ];

        let result = points_to(start, end);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_points_along() {
        let path = "498,4 -> 498,6 -> 496,6";

        let expected = vec![
            Point2d::new(498, 4),
            Point2d::new(498, 5),
            Point2d::new(498, 6),
            Point2d::new(497, 6),
            Point2d::new(496, 6),
        ];

        let result = points_along(path);

        assert_eq!(result, expected);
    }
}
