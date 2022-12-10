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
    head: Point2d<i32>,
    tail: Point2d<i32>,
}

impl Rope {
    pub fn apply_motions(
        &mut self,
        motions: &[Motion],
    ) -> (HashSet<Point2d<i32>>, HashSet<Point2d<i32>>) {
        motions.iter().fold(
            (HashSet::new(), HashSet::new()),
            |(h_acc, t_acc), &motion| {
                let (h, t) = self.apply_motion(motion);

                (
                    h_acc.union(&h).cloned().collect(),
                    t_acc.union(&t).cloned().collect(),
                )
            },
        )
    }

    fn apply_motion(&mut self, motion: Motion) -> (HashSet<Point2d<i32>>, HashSet<Point2d<i32>>) {
        let mut head_locations = HashSet::from([self.head]);
        let mut tail_locations = HashSet::from([self.tail]);

        let destination = self.head.add(&motion.as_offset());
        let step_offset = motion.as_normalized_offset();

        while self.head != destination {
            let old_head = self.head;
            self.head = self.head.add(&step_offset);

            let tail_within_x_range = self.head.x.abs_diff(self.tail.x) <= 1;
            let tail_within_y_range = self.head.y.abs_diff(self.tail.y) <= 1;

            if !tail_within_x_range || !tail_within_y_range {
                self.tail = old_head;
            }

            head_locations.insert(self.head);
            tail_locations.insert(self.tail);
        }

        (head_locations, tail_locations)
    }
}

impl Rope {
    pub fn new() -> Rope {
        Rope {
            head: Point2d::new(0, 0),
            tail: Point2d::new(0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_apply_right() {
        let mut rope = Rope::new();

        rope.head = Point2d::new(1, 3);
        rope.tail = Point2d::new(2, 4);

        let expected = (
            HashSet::from([
                Point2d::new(1, 3),
                Point2d::new(2, 3),
                Point2d::new(3, 3),
                Point2d::new(4, 3),
                Point2d::new(5, 3),
            ]),
            HashSet::from([Point2d::new(2, 4), Point2d::new(3, 3), Point2d::new(4, 3)]),
        );

        let result: (HashSet<Point2d<i32>>, HashSet<Point2d<i32>>) =
            rope.apply_motion(Motion::Right(4));

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

        let mut rope = Rope::new();

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

        let result: HashSet<Point2d<i32>> = rope.apply_motions(&input).1;

        assert_eq!(result, expected);
    }
}
