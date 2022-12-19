use std::collections::HashSet;

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
pub struct Cave {
    rocks: HashSet<Point2d<i32>>,
    sand: HashSet<Point2d<i32>>,
    sand_starting_point: Point2d<i32>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
}

impl Cave {
    pub fn new() -> Cave {
        Cave {
            rocks: HashSet::new(),
            sand: HashSet::new(),
            sand_starting_point: Point2d::new(500, 0),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
        }
    }

    pub fn add_rock(&mut self, rock_definition: &str) {
        self.rocks.extend(points_along(rock_definition).iter());

        for point in self.rocks.iter() {
            if point.x < self.x_bounds.0 {
                self.x_bounds.0 = point.x;
            }

            if point.x > self.x_bounds.1 {
                self.x_bounds.1 = point.x;
            }

            if point.y < self.y_bounds.0 {
                self.y_bounds.0 = point.y;
            }

            if point.y > self.y_bounds.1 {
                self.y_bounds.1 = point.y;
            }
        }
    }

    pub fn drop_sand(&mut self) -> Option<Point2d<i32>> {
        let mut current_point = self.sand_starting_point.clone();

        while self.is_in_bounds(&current_point) {
            let point_down = current_point.add_t((0, 1));
            let point_down_and_left = current_point.add_t((-1, 1));
            let point_down_and_right = current_point.add_t((1, 1));

            if !self.is_occupied(&point_down) {
                current_point = point_down;
            } else if !self.is_occupied(&point_down_and_left) {
                current_point = point_down_and_left;
            } else if !self.is_occupied(&point_down_and_right) {
                current_point = point_down_and_right;
            } else {
                break;
            }
        }

        if self.is_in_bounds(&current_point) {
            self.sand.insert(current_point.clone());

            Some(current_point)
        } else {
            None
        }
    }

    fn is_in_bounds(&self, point: &Point2d<i32>) -> bool {
        let in_x_bounds = (self.x_bounds.0 <= point.x) && (point.x <= self.x_bounds.1);
        let in_y_bounds = (self.y_bounds.0 <= point.y) && (point.y <= self.y_bounds.1);

        in_x_bounds && in_y_bounds
    }

    fn is_occupied(&self, point: &Point2d<i32>) -> bool {
        self.rocks.contains(&point) || self.sand.contains(&point)
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

    #[test]
    fn test_drop_sand_once() {
        let input = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let mut cave = Cave::new();

        input.iter().for_each(|line| cave.add_rock(&line));

        let expected = Some(Point2d::new(500, 8));

        let result = cave.drop_sand();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_sand_24_times() {
        let input = vec![
            String::from("498,4 -> 498,6 -> 496,6"),
            String::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let mut cave = Cave::new();

        input.iter().for_each(|line| cave.add_rock(&line));

        (0..23).for_each(|_| {
            cave.drop_sand();
        });

        let expected = Some(Point2d::new(495, 8));

        let result = cave.drop_sand();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_sand_25_times() {
        let input = vec![
            string::from("498,4 -> 498,6 -> 496,6"),
            string::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let mut cave = Cave::new();

        input.iter().for_each(|line| cave.add_rock(&line));

        (0..24).for_each(|_| {
            cave.drop_sand();
        });

        let expected = None;

        let result = cave.drop_sand();

        assert_eq!(result, expected);
    }
}
