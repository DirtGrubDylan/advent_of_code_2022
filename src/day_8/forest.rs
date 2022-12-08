#[derive(Debug, PartialEq)]
pub struct Forest {
    trees: Vec<Vec<u32>>,
    length: usize,
    width: usize,
}

impl From<&Vec<String>> for Forest {
    fn from(input: &Vec<String>) -> Forest {
        let trees: Vec<Vec<u32>> = input
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let length = trees.len();
        let width = trees.get(0).map_or(0, |row| row.len());

        Forest { trees, length, width }
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
}
