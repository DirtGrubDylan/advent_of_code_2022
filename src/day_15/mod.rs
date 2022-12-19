mod emergency_sensor_system;

use crate::util::file_reader::to_string_vector;

use emergency_sensor_system::Signal;

pub fn run() {
    let input = to_string_vector("inputs/day_15.txt").expect("Something went wrong Day 15!");

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unimplemented!()
    }
}
