use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, PartialEq)]
pub struct Forest {
    trees: Vec<Vec<u32>>,
    length: usize,
    width: usize,
}

impl Forest {
    pub fn tree_visibility(&self) -> HashMap<(usize, usize), HashSet<Direction>> {
        let mut result = HashMap::new();

        for row in 0..self.length {
            for col in 0..self.width {
                result.entry((row, col)).or_insert(HashSet::new());
            }
        }

        for location in self.tree_visibility_from(Direction::Top) {
            result.entry(location).and_modify(|directions| {
                directions.insert(Direction::Top);
            });
        }

        for location in self.tree_visibility_from(Direction::Right) {
            result.entry(location).and_modify(|directions| {
                directions.insert(Direction::Right);
            });
        }

        for location in self.tree_visibility_from(Direction::Bottom) {
            result.entry(location).and_modify(|directions| {
                directions.insert(Direction::Bottom);
            });
        }

        for location in self.tree_visibility_from(Direction::Left) {
            result.entry(location).and_modify(|directions| {
                directions.insert(Direction::Left);
            });
        }

        result
    }

    pub fn tree_scenic_scores(&self) -> HashMap<(usize, usize), usize> {
        let mut result = HashMap::new();

        for (location, score) in self.tree_scenic_scores_from(Direction::Top) {
            result.entry(location).or_insert(score);
        }

        for (location, score) in self.tree_scenic_scores_from(Direction::Right) {
            result.entry(location).and_modify(|value| *value *= score);
        }

        for (location, score) in self.tree_scenic_scores_from(Direction::Bottom) {
            result.entry(location).and_modify(|value| *value *= score);
        }

        for (location, score) in self.tree_scenic_scores_from(Direction::Left) {
            result.entry(location).and_modify(|value| *value *= score);
        }

        result
    }

    fn tree_visibility_from(&self, direction: Direction) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();

        let mut max_height_so_far = None;

        for count in 0..(self.length * self.width) {
            let (row, col) = self.next_tree_from(direction, count);

            if self.line_count(direction, count) == 0 {
                max_height_so_far = None;
            }

            let tree_height = *self
                .trees
                .get(row)
                .map(|row| row.get(col))
                .flatten()
                .expect(&format!("Couldn't get tree at: {:?}", (row, col)));

            match max_height_so_far {
                Some(max_height) if tree_height <= max_height => (),
                _ => {
                    result.insert((row, col));
                    max_height_so_far = Some(tree_height);
                }
            }
        }

        result
    }

    fn tree_scenic_scores_from(&self, direction: Direction) -> HashMap<(usize, usize), usize> {
        let mut result = HashMap::new();

        let mut heights_last_seen = HashMap::new();

        for tree in 0..(self.length * self.width) {
            let (row, col) = self.next_tree_from(direction, tree);

            let tree_in_line = self.line_count(direction, tree);

            if tree_in_line == 0 {
                heights_last_seen = HashMap::new();
            }

            let tree_height = *self
                .trees
                .get(row)
                .map(|line| line.get(col))
                .flatten()
                .expect(&format!("Couldn't get tree at: {:?}", (row, col)));

            let score = (tree_height..10)
                .filter_map(|height| {
                    heights_last_seen
                        .get(&height)
                        .map(|last_seen| tree_in_line - last_seen)
                })
                .min()
                .unwrap_or(tree_in_line);

            result.insert((row, col), score);

            heights_last_seen.insert(tree_height, tree_in_line);
        }

        result
    }

    fn next_tree_from(&self, direction: Direction, count: usize) -> (usize, usize) {
        match direction {
            Direction::Top => (count % self.length, count / self.length),
            Direction::Right => (count / self.width, self.width - 1 - count % self.width),
            Direction::Bottom => (self.length - 1 - count % self.length, count / self.length),
            Direction::Left => (count / self.width, count % self.width),
        }
    }

    fn line_count(&self, direction: Direction, count: usize) -> usize {
        match direction {
            Direction::Top => count % self.length,
            Direction::Right => count % self.width,
            Direction::Bottom => count % self.length,
            Direction::Left => count % self.width,
        }
    }
}

impl From<&Vec<String>> for Forest {
    fn from(input: &Vec<String>) -> Forest {
        let trees: Vec<Vec<u32>> = input
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let length = trees.len();
        let width = trees.get(0).map_or(0, |row| row.len());

        Forest {
            trees,
            length,
            width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let expected = Forest {
            trees: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
            width: 5,
            length: 5,
        };

        let result = Forest::from(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_visibility_from_top() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = HashSet::from([
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 3),
        ]);

        let result = forest.tree_visibility_from(Direction::Top);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_visibility_from_right() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = HashSet::from([
            (0, 3),
            (0, 4),
            (1, 2),
            (1, 4),
            (2, 0),
            (2, 1),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 3),
            (4, 4),
        ]);

        let result = forest.tree_visibility_from(Direction::Right);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_visibility() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = HashMap::from([
            ((0, 0), HashSet::from([Direction::Top, Direction::Left])),
            ((0, 1), HashSet::from([Direction::Top])),
            ((0, 2), HashSet::from([Direction::Top])),
            (
                (0, 3),
                HashSet::from([Direction::Top, Direction::Right, Direction::Left]),
            ),
            ((0, 4), HashSet::from([Direction::Top, Direction::Right])),
            ((1, 0), HashSet::from([Direction::Left])),
            ((1, 1), HashSet::from([Direction::Top, Direction::Left])),
            ((1, 2), HashSet::from([Direction::Top, Direction::Right])),
            ((1, 3), HashSet::from([])),
            ((1, 4), HashSet::from([Direction::Right])),
            (
                (2, 0),
                HashSet::from([
                    Direction::Top,
                    Direction::Right,
                    Direction::Bottom,
                    Direction::Left,
                ]),
            ),
            ((2, 1), HashSet::from([Direction::Right])),
            ((2, 2), HashSet::from([])),
            ((2, 3), HashSet::from([Direction::Right])),
            ((2, 4), HashSet::from([Direction::Right])),
            ((3, 0), HashSet::from([Direction::Left])),
            ((3, 1), HashSet::from([])),
            ((3, 2), HashSet::from([Direction::Bottom, Direction::Left])),
            ((3, 3), HashSet::from([])),
            (
                (3, 4),
                HashSet::from([
                    Direction::Top,
                    Direction::Right,
                    Direction::Bottom,
                    Direction::Left,
                ]),
            ),
            ((4, 0), HashSet::from([Direction::Bottom, Direction::Left])),
            ((4, 1), HashSet::from([Direction::Bottom, Direction::Left])),
            ((4, 2), HashSet::from([Direction::Bottom])),
            (
                (4, 3),
                HashSet::from([
                    Direction::Top,
                    Direction::Right,
                    Direction::Bottom,
                    Direction::Left,
                ]),
            ),
            ((4, 4), HashSet::from([Direction::Right, Direction::Bottom])),
        ]);

        let result = forest.tree_visibility();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_scenic_scores_from_bottom() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = vec![2, 1];

        let results = forest.tree_scenic_scores_from(Direction::Bottom);

        let result = vec![
            results.get(&(1, 2)).cloned().unwrap(),
            results.get(&(3, 2)).cloned().unwrap(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_scenic_scores_from_left() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = vec![1, 2];

        let results = forest.tree_scenic_scores_from(Direction::Left);

        let result = vec![
            results.get(&(1, 2)).cloned().unwrap(),
            results.get(&(3, 2)).cloned().unwrap(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tree_scenic_scores() {
        let input = vec![
            String::from("30373"),
            String::from("25512"),
            String::from("65332"),
            String::from("33549"),
            String::from("35390"),
        ];

        let forest = Forest::from(&input);

        let expected = vec![4, 8];

        let results = forest.tree_scenic_scores();

        let result = vec![
            results.get(&(1, 2)).cloned().unwrap(),
            results.get(&(3, 2)).cloned().unwrap(),
        ];

        assert_eq!(result, expected);
    }
}
