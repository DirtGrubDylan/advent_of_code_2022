use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
struct Move {
    point: Point2d<i32>,
    distance_traveled: usize,
    points_seen: Vec<Point2d<i32>>,
}

impl Move {
    fn new(point: &Point2d<i32>) -> Move {
        Move {
            point: *point,
            distance_traveled: 0,
            points_seen: vec![],
        }
    }

    fn next_move(&self, x_delta: i32, y_delta: i32) -> Move {
        let distance_traveled =
            self.distance_traveled + (x_delta.abs() as usize) + (y_delta.abs() as usize);
        let mut points_seen: Vec<Point2d<i32>> = self.points_seen.iter().cloned().collect();

        points_seen.push(self.point);

        Move {
            point: self.point.add_t((x_delta, y_delta)),
            distance_traveled,
            points_seen,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HeightMap {
    pub start: Point2d<i32>,
    pub end: Point2d<i32>,
    points: HashMap<Point2d<i32>, char>,
}

impl From<&Vec<String>> for HeightMap {
    fn from(input: &Vec<String>) -> HeightMap {
        let mut result = HeightMap::new();

        for (row_index, row) in input.iter().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                let point = Point2d::new(col_index as i32, row_index as i32);

                match col {
                    'S' => {
                        result.start = point;
                        result.points.insert(point, 'a');
                    }
                    'E' => {
                        result.end = point;
                        result.points.insert(point, 'z');
                    }
                    _ => {
                        result.points.insert(point, col);
                    }
                }
            }
        }

        result
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

    pub fn get_shortest_hiking_path(&self) -> Vec<Point2d<i32>> {
        let lowest_points: Vec<Point2d<i32>> = self
            .points
            .iter()
            .filter(|(_, height)| **height == 'a')
            .map(|(point, _)| *point)
            .collect();

        let mut min_path_so_far = vec![];
        let mut already_started = HashSet::new();

        for start in lowest_points.into_iter() {
            let path = self.get_shortest_path_starting_from(
                start,
                min_path_so_far.len(),
                &already_started,
            );

            if path.is_empty() {
                continue;
            }

            if min_path_so_far.is_empty() || (path.len() < min_path_so_far.len()) {
                min_path_so_far = path;
            }

            already_started.insert(start);
        }

        min_path_so_far
    }

    pub fn get_shortest_path(&self) -> Vec<Point2d<i32>> {
        self.get_shortest_path_starting_from(self.start, self.points.len(), &HashSet::new())
    }

    pub fn get_shortest_path_starting_from(
        &self,
        start: Point2d<i32>,
        max_distance: usize,
        already_started_points: &HashSet<Point2d<i32>>,
    ) -> Vec<Point2d<i32>> {
        let mut added_points: HashSet<Point2d<i32>> = HashSet::from([start]);
        let mut move_queue: VecDeque<Move> = VecDeque::new();
        let mut result = vec![];

        let first_move = Move::new(&start);

        move_queue.push_back(first_move);
        added_points.extend(already_started_points);

        while let Some(current_move) = move_queue.pop_front() {
            if current_move.point == self.end {
                result = current_move.points_seen;
                result.push(self.end);

                break;
            }

            if (current_move.distance_traveled == max_distance) && (max_distance != 0) {
                result = vec![];

                break;
            }

            let mut next_moves: Vec<Move> = vec![
                current_move.next_move(0, -1),
                current_move.next_move(1, 0),
                current_move.next_move(0, 1),
                current_move.next_move(-1, 0),
            ]
            .into_iter()
            .filter(|next_move| self.points.contains_key(&next_move.point))
            .filter(|next_move| !added_points.contains(&next_move.point))
            .filter(|next_move| self.height_difference(&current_move.point, &next_move.point) <= 1)
            .collect();

            while let Some(next_move) = next_moves.pop() {
                added_points.insert(next_move.point);

                move_queue.push_back(next_move);
            }
        }

        result
    }

    fn height_difference(&self, current_point: &Point2d<i32>, next_point: &Point2d<i32>) -> i32 {
        let lhs = self.points.get(current_point).map_or(0, |v| *v as i32);
        let rhs = self.points.get(next_point).map_or(0, |v| *v as i32);

        rhs - lhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_next() {
        let current_move = Move::new(&Point2d::new(0, 0));

        let expected = Move {
            point: Point2d::new(1, -1),
            distance_traveled: 2,
            points_seen: vec![Point2d::new(0, 0), Point2d::new(0, -1)],
        };

        let result = current_move.next_move(0, -1).next_move(1, 0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let result = HeightMap::from(&input);

        assert_eq!(result.start, Point2d::new(0, 0));
        assert_eq!(result.end, Point2d::new(5, 2));
        assert_eq!(result.points.len(), 40);
    }

    #[test]
    fn test_get_shortest_hiking_path() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let height_map = HeightMap::from(&input);

        // ...v<<<<
        // ...vv<<^
        // ...v>E^^
        // .>v>>>^^
        // >^>>>>>^
        let expected = vec![
            Point2d::new(0, 4),
            Point2d::new(1, 4),
            Point2d::new(1, 3),
            Point2d::new(2, 3),
            Point2d::new(2, 4),
            Point2d::new(3, 4),
            Point2d::new(4, 4),
            Point2d::new(5, 4),
            Point2d::new(6, 4),
            Point2d::new(7, 4),
            Point2d::new(7, 3),
            Point2d::new(7, 2),
            Point2d::new(7, 1),
            Point2d::new(7, 0),
            Point2d::new(6, 0),
            Point2d::new(5, 0),
            Point2d::new(4, 0),
            Point2d::new(3, 0),
            Point2d::new(3, 1),
            Point2d::new(3, 2),
            Point2d::new(3, 3),
            Point2d::new(4, 3),
            Point2d::new(5, 3),
            Point2d::new(6, 3),
            Point2d::new(6, 2),
            Point2d::new(6, 1),
            Point2d::new(5, 1),
            Point2d::new(4, 1),
            Point2d::new(4, 2),
            Point2d::new(5, 2),
        ];

        let result = height_map.get_shortest_hiking_path();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_shortest_path() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let height_map = HeightMap::from(&input);

        // v..v<<<<
        // >v.vv<<^
        // .>vv>E^^
        // ..v>>>^^
        // ..>>>>>^
        let expected = vec![
            Point2d::new(0, 0),
            Point2d::new(0, 1),
            Point2d::new(1, 1),
            Point2d::new(1, 2),
            Point2d::new(1, 3),
            Point2d::new(2, 3),
            Point2d::new(2, 4),
            Point2d::new(3, 4),
            Point2d::new(4, 4),
            Point2d::new(5, 4),
            Point2d::new(6, 4),
            Point2d::new(7, 4),
            Point2d::new(7, 3),
            Point2d::new(7, 2),
            Point2d::new(7, 1),
            Point2d::new(7, 0),
            Point2d::new(6, 0),
            Point2d::new(5, 0),
            Point2d::new(4, 0),
            Point2d::new(3, 0),
            Point2d::new(3, 1),
            Point2d::new(3, 2),
            Point2d::new(3, 3),
            Point2d::new(4, 3),
            Point2d::new(5, 3),
            Point2d::new(6, 3),
            Point2d::new(6, 2),
            Point2d::new(6, 1),
            Point2d::new(5, 1),
            Point2d::new(4, 1),
            Point2d::new(4, 2),
            Point2d::new(5, 2),
        ];

        let result = height_map.get_shortest_path();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_height_difference() {
        let input = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];

        let height_map = HeightMap::from(&input);

        let expected_1 = 0;
        let expected_2 = -1;
        let expected_3 = 25;

        let result_1 = height_map.height_difference(&Point2d::new(0, 0), &Point2d::new(0, 1));
        let result_2 = height_map.height_difference(&Point2d::new(1, 1), &Point2d::new(0, 0));
        let result_3 = height_map.height_difference(&Point2d::new(0, 0), &Point2d::new(5, 2));

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
    }
}
