mod emergency_sensor_system;

use std::{collections::HashSet, ops::RangeInclusive};

use crate::util::file_reader::to_string_vector;

use emergency_sensor_system::Signal;

pub fn run() {
    let input = to_string_vector("inputs/day_15.txt").expect("Something went wrong Day 15!");

    let signals: Vec<Signal> = input.into_iter().map(|line| Signal::from(&line)).collect();

    println!("Day 15, Part 1: {}", part_1(&signals, 2_000_000));
}

fn part_1(signals: &[Signal], row: i32) -> usize {
    let ranges = get_ranges(signals, row);

    let mut current_range = ranges.get(0).unwrap().clone();
    let mut number_of_spaces = 0;

    for range in ranges.into_iter().skip(1) {
        if current_range.end() < range.start() {
            number_of_spaces += current_range.count();

            current_range = range;
        } else if current_range.end() < range.end() {
            current_range = (*current_range.start())..=(*range.end());
        }
    }

    number_of_spaces + current_range.count() - number_of_beacons_on_row(signals, row)
}

fn get_ranges(signals: &[Signal], row: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<RangeInclusive<i32>> = signals
        .iter()
        .filter_map(|signal| signal.impossible_beacon_column_incl_ranges(row))
        .collect();

    ranges.sort_by(|range, other| {
        range
            .start()
            .cmp(other.start())
            .then(other.end().cmp(range.end()))
    });

    ranges.dedup_by(|range, other| range.start().eq(other.start()));

    ranges
}

fn number_of_beacons_on_row(signals: &[Signal], row: i32) -> usize {
    let mut seen_beacons = HashSet::new();

    for signal in signals.iter() {
        let beacon = signal.closest_beacon_point;

        if (beacon.y == row) && !seen_beacons.contains(&beacon) {
            seen_beacons.insert(beacon);
        }
    }

    seen_beacons.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        let signals: Vec<Signal> = input.into_iter().map(|line| Signal::from(&line)).collect();

        let expected = 26;

        let result = part_1(&signals, 10);

        assert_eq!(result, expected);
    }
}
