mod emergency_sensor_system;

use std::{collections::HashSet, ops::RangeInclusive};

use crate::util::file_reader::to_string_vector;

use emergency_sensor_system::Signal;

pub fn run() {
    let input = to_string_vector("inputs/day_15.txt").expect("Something went wrong Day 15!");

    let signals: Vec<Signal> = input.into_iter().map(|line| Signal::from(&line)).collect();

    println!("Day 15, Part 1: {}", part_1(&signals, 2_000_000));
    println!("Day 15, Part 2: {:?}", part_2(&signals, 4_000_000));
}

fn part_1(signals: &[Signal], row: i32) -> usize {
    let number_of_spaces: usize = get_ranges(signals, row, None)
        .iter()
        .map(|range| range.clone().count())
        .sum();

    number_of_spaces - number_of_beacons_on_row(signals, row)
}

fn part_2(signals: &[Signal], maximum_value: i32) -> Option<i64> {
    (0..=maximum_value)
        .rev()
        .map(|row| {
            (
                row as i64,
                get_ranges(signals, row, Some((0, maximum_value))),
            )
        })
        .find(|(_, ranges)| {
            ranges.len() > 1
                || ranges.first().map_or(maximum_value, |range| *range.end()) != maximum_value
                || ranges.first().map_or(0, |range| *range.start()) != 0
        })
        .map(|(row, ranges)| ranges.first().map(|range| ((*range.end() + 1) as i64, row)))
        .flatten()
        .map(|(col, row)| col * 4_000_000 + row)
}

fn get_ranges(
    signals: &[Signal],
    row: i32,
    bounds: Option<(i32, i32)>,
) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<RangeInclusive<i32>> = signals
        .iter()
        .filter_map(|signal| {
            if let Some(bound) = bounds {
                signal.impossible_beacon_column_bounded_ranges(row, bound)
            } else {
                signal.impossible_beacon_column_incl_ranges(row)
            }
        })
        .collect();

    ranges.sort_by(|range, other| {
        range
            .start()
            .cmp(other.start())
            .then(other.end().cmp(range.end()))
    });

    let mut current_range = ranges.remove(0);

    let mut result = vec![];

    for range in ranges.into_iter() {
        if (*current_range.end() + 1) < *range.start() {
            result.push(current_range.clone());

            current_range = range;
        } else if current_range.end() < range.end() {
            current_range = (*current_range.start())..=(*range.end());
        }
    }

    result.push(current_range.clone());

    result
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

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        let signals: Vec<Signal> = input.into_iter().map(|line| Signal::from(&line)).collect();

        let expected = Some(56_000_011);

        let result = part_2(&signals, 20);

        assert_eq!(result, expected);
    }
}
