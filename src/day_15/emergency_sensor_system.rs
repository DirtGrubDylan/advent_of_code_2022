use std::ops::RangeInclusive;

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq)]
pub struct Signal {
    pub closest_beacon_point: Point2d<i32>,
    point: Point2d<i32>,
    distance_to_closest_beacon: i32,
}

impl Signal {
    fn new(point: Point2d<i32>, closest_beacon_point: Point2d<i32>) -> Signal {
        let distance_to_closest_beacon = (point.x.abs_diff(closest_beacon_point.x)
            + point.y.abs_diff(closest_beacon_point.y))
            as i32;

        Signal {
            closest_beacon_point,
            point,
            distance_to_closest_beacon,
        }
    }

    /*
    For each signal S, you have beacon B which is M distance away.

    The number of x's on row y is: 2 * M - 2 * |Sy - y| + 1

    The inclusive range is: [Sx + |Sy - y| - M, Sx + M - |Sy - y|]

    The range is best used to detect overlaps between signals
    */
    pub fn impossible_beacon_column_incl_ranges(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let y_distance = self.point.y.abs_diff(row) as i32;

        let left_bound = self.point.x + y_distance - self.distance_to_closest_beacon;
        let right_bound = self.point.x + self.distance_to_closest_beacon - y_distance;

        if left_bound <= right_bound {
            Some(left_bound..=right_bound)
        } else {
            None
        }
    }
}

impl From<&String> for Signal {
    fn from(input: &String) -> Signal {
        let temp_input = input
            .replace("Sensor at x=", "")
            .replace(" closest beacon is at x=", "")
            .replace(" y=", "");

        let (point_input, beacon_input) = temp_input.split_once(':').unwrap();

        let point = point_input
            .split_once(',')
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .map(|(x, y)| Point2d::new(x, y))
            .unwrap();

        let beacon = beacon_input
            .split_once(',')
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .map(|(x, y)| Point2d::new(x, y))
            .unwrap();

        Signal::new(point, beacon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = String::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");

        let expected = Signal {
            point: Point2d::new(2, 18),
            closest_beacon_point: Point2d::new(-2, 15),
            distance_to_closest_beacon: 7,
        };

        let result = Signal::from(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_impossible_beacon_column_incl_ranges_none() {
        let signal = Signal::new(Point2d::new(2, 18), Point2d::new(-2, 15));

        let expected = None;

        let result = signal.impossible_beacon_column_incl_ranges(10);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_impossible_beacon_column_incl_ranges_some() {
        let signal = Signal::new(Point2d::new(8, 7), Point2d::new(2, 10));

        let expected = Some(2..=14);

        let result = signal.impossible_beacon_column_incl_ranges(10);

        assert_eq!(result, expected);
    }
}
