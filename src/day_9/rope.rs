use std::collections::HashSet;

use crate::util::{location::Location, point_2d::Point2d};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Motion {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl Motion {
    fn as_offset(&self) -> Point2d<i32> {
        match self {
            &Motion::Up(y_offset) => Point2d::new(0, y_offset as i32),
            &Motion::Right(x_offset) => Point2d::new(x_offset as i32, 0),
            &Motion::Down(y_offset) => Point2d::new(0, -(y_offset as i32)),
            &Motion::Left(x_offset) => Point2d::new(-(x_offset as i32), 0),
        }
    }

    fn as_normalized_offset(&self) -> Point2d<i32> {
        match self {
            &Motion::Up(_) => Point2d::new(0, 1),
            &Motion::Right(_) => Point2d::new(1, 0),
            &Motion::Down(_) => Point2d::new(0, -1),
            &Motion::Left(_) => Point2d::new(-1, 0),
        }
    }
}

impl From<&String> for Motion {
    fn from(input: &String) -> Motion {
        let (direction, magnitude) = input
            .split_once(' ')
            .map(|(d, m)| (d, m.parse().unwrap()))
            .unwrap();

        match direction {
            "U" => Motion::Up(magnitude),
            "R" => Motion::Right(magnitude),
            "D" => Motion::Down(magnitude),
            "L" => Motion::Left(magnitude),
            _ => panic!("Fux: {:?}", (direction, magnitude)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rope {
    length: usize,
    knots: Vec<Point2d<i32>>,
}

impl Rope {
    pub fn apply_motions(&mut self, motions: &[Motion]) -> HashSet<Point2d<i32>> {
        motions.iter().fold(HashSet::new(), |acc, &motion| {
            let tail_positions = self.apply_motion(motion);

            acc.union(&tail_positions).cloned().collect()
        })
    }

    fn apply_motion(&mut self, motion: Motion) -> HashSet<Point2d<i32>> {
        let mut tail_locations = HashSet::from([*self.knots.last().unwrap()]);

        let destination = self.knots.first().unwrap().add(&motion.as_offset());
        let step_offset = motion.as_normalized_offset();

        while *self.knots.first().unwrap() != destination {
            let head = self.knots.first_mut().unwrap();

            let mut new_location = head.add(&step_offset);

            *head = new_location;

            for (knot_index, knot) in self.knots.iter_mut().enumerate().skip(1) {
                let knot_within_x_range = new_location.x.abs_diff(knot.x) <= 1;
                let knot_within_y_range = new_location.y.abs_diff(knot.y) <= 1;

                if knot_within_x_range && knot_within_y_range {
                    break;
                }

                let mut temp_location = *knot;

                if knot.x < new_location.x {
                    temp_location.x += 1;
                } else if knot.x > new_location.x {
                    temp_location.x -= 1;
                }

                if knot.y < new_location.y {
                    temp_location.y += 1;
                } else if knot.y > new_location.y {
                    temp_location.y -= 1;
                }

                *knot = temp_location;
                new_location = temp_location;

                if knot_index == self.length - 1 {
                    tail_locations.insert(*knot);
                }
            }
        }

        tail_locations
    }
}

impl Rope {
    pub fn new(length: usize) -> Rope {
        Rope {
            length,
            knots: vec![Point2d::new(0, 0); length],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_apply_right() {
        let mut rope = Rope::new(2);

        rope.knots = vec![Point2d::new(1, 3), Point2d::new(2, 4)];

        let expected = HashSet::from([Point2d::new(2, 4), Point2d::new(3, 3), Point2d::new(4, 3)]);

        let result: HashSet<Point2d<i32>> = rope.apply_motion(Motion::Right(4));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rope_apply_motions() {
        let input = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];

        let mut rope = Rope::new(2);

        let expected = HashSet::from([
            Point2d::new(0, 0),
            Point2d::new(1, 0),
            Point2d::new(2, 0),
            Point2d::new(3, 0),
            Point2d::new(4, 1),
            Point2d::new(1, 2),
            Point2d::new(2, 2),
            Point2d::new(3, 2),
            Point2d::new(4, 2),
            Point2d::new(3, 3),
            Point2d::new(4, 3),
            Point2d::new(2, 4),
            Point2d::new(3, 4),
        ]);

        let result: HashSet<Point2d<i32>> = rope.apply_motions(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rope_apply_motions_10_knots_no_movement() {
        let input = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];

        let mut rope = Rope::new(10);

        let expected = HashSet::from([Point2d::new(0, 0)]);

        let result: HashSet<Point2d<i32>> = rope.apply_motions(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rope_apply_motions_10_knots() {
        let input = vec![
            Motion::Right(5),
            Motion::Up(8),
            Motion::Left(8),
            Motion::Down(3),
            Motion::Right(17),
            Motion::Down(10),
            Motion::Left(25),
            Motion::Up(20),
        ];

        let mut rope = Rope::new(10);

        let expected = HashSet::from([
            Point2d::new(0, 0),
            Point2d::new(1, 1),
            Point2d::new(2, 2),
            Point2d::new(1, 3),
            Point2d::new(2, 4),
            Point2d::new(3, 5),
            Point2d::new(4, 5),
            Point2d::new(5, 5),
            Point2d::new(6, 4),
            Point2d::new(7, 3),
            Point2d::new(8, 2),
            Point2d::new(9, 1),
            Point2d::new(10, 0),
            Point2d::new(9, -1),
            Point2d::new(8, -2),
            Point2d::new(7, -3),
            Point2d::new(6, -4),
            Point2d::new(5, -5),
            Point2d::new(4, -5),
            Point2d::new(3, -5),
            Point2d::new(2, -5),
            Point2d::new(1, -5),
            Point2d::new(0, -5),
            Point2d::new(-1, -5),
            Point2d::new(-2, -5),
            Point2d::new(-3, -4),
            Point2d::new(-4, -3),
            Point2d::new(-5, -2),
            Point2d::new(-6, -1),
            Point2d::new(-7, 0),
            Point2d::new(-8, 1),
            Point2d::new(-9, 2),
            Point2d::new(-10, 3),
            Point2d::new(-11, 4),
            Point2d::new(-11, 5),
            Point2d::new(-11, 6),
        ]);

        let result: HashSet<Point2d<i32>> = rope.apply_motions(&input);

        println!("{:?}", result.symmetric_difference(&expected));

        assert_eq!(result.len(), expected.len());
        assert_eq!(result, expected);
    }
}
