use std::collections::HashSet;

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
pub enum Motion {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl From<&String> for Motion {
    fn from(input: &String) -> Motion {
        unimplemented!()
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
        unimplemented!()
    }

    fn apply_motion(&mut self, motion: Motion) -> (HashSet<Point2d<i32>>, HashSet<Point2d<i32>>) {
        let mut head_locations = HashSet::from([self.head]);
        let mut tail_locations = HashSet::from([self.tail]);

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
            ]),
            HashSet::from([Point2d::new(2, 4), Point2d::new(3, 3)]),
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
