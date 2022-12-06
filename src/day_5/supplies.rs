use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl From<&String> for Instruction {
    fn from(input: &String) -> Instruction {
        let values: Vec<usize> = input.split(' ').filter_map(|s| s.parse().ok()).collect();

        Instruction {
            quantity: *(values
                .get(0)
                .expect(&format!("Missing quantity: {}", input))),
            from: *(values.get(1).expect(&format!("Missing from: {}", input))),
            to: *(values.get(2).expect(&format!("Missing to: {}", input))),
        }
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Instruction {
        let values: Vec<usize> = input.split(' ').filter_map(|s| s.parse().ok()).collect();

        Instruction {
            quantity: *(values
                .get(0)
                .expect(&format!("Missing quantity: {}", input))),
            from: *(values.get(1).expect(&format!("Missing from: {}", input))),
            to: *(values.get(2).expect(&format!("Missing to: {}", input))),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Supplies {
    stacks: Vec<Vec<char>>,
}

impl From<&Vec<String>> for Supplies {
    fn from(input: &Vec<String>) -> Supplies {
        let temp: Vec<Vec<char>> = input
            .iter()
            .rev()
            .skip(1)
            .map(|line| line.chars().skip(1).step_by(4).collect())
            .collect();

        let mut stacks: Vec<Vec<char>> = Vec::new();

        for _ in 0..temp[0].len() {
            stacks.push(Vec::new());
        }

        for row_index in 0..temp.len() {
            let temp_row = &temp[row_index];

            for col_index in 0..temp_row.len() {
                let value = temp_row[col_index];
                let stack = &mut stacks[col_index];

                if value != ' ' {
                    stack.push(value);
                }
            }
        }

        Supplies { stacks }
    }
}

#[derive(Debug, PartialEq)]
pub struct Crane {
    supplies: Supplies,
}

impl Crane {
    pub fn execute_9000(&mut self, instruction: &Instruction) {
        let from_row = &mut self.supplies.stacks[instruction.from - 1];
        let mut temp = Vec::new();

        for _ in 0..instruction.quantity {
            if let Some(value) = from_row.pop() {
                temp.push(value);
            }
        }

        let to_row = &mut self.supplies.stacks[instruction.to - 1];

        to_row.append(&mut temp);
    }

    pub fn execute_9001(&mut self, instruction: &Instruction) {
        let from_row = &mut self.supplies.stacks[instruction.from - 1];
        let mut temp = VecDeque::new();

        for _ in 0..instruction.quantity {
            if let Some(value) = from_row.pop() {
                temp.push_front(value);
            }
        }

        let to_row = &mut self.supplies.stacks[instruction.to - 1];

        to_row.extend_from_slice(temp.make_contiguous());
    }

    pub fn top_supplies(&self) -> Vec<char> {
        self.supplies
            .stacks
            .iter()
            .map(|stack| stack.last().map_or(' ', |id| *id))
            .collect()
    }
}

impl From<&Vec<String>> for Crane {
    fn from(input: &Vec<String>) -> Crane {
        Crane {
            supplies: Supplies::from(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_instruction() {
        let input = "move 11 from 4 to 7";

        let expected = Instruction {
            quantity: 11,
            from: 4,
            to: 7,
        };

        let result = Instruction::from(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_supplies() {
        let input = vec![
            String::from("    [D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ];

        let expected = Supplies {
            stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };

        let result = Supplies::from(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_crane_execute_9000() {
        let input = vec![
            String::from("    [D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ];

        let instruction = Instruction::from("move 3 from 2 to 3");

        let mut crane = Crane::from(&input);

        let expected_supplies = Supplies {
            stacks: vec![vec!['Z', 'N'], vec![], vec!['P', 'D', 'C', 'M']],
        };

        crane.execute_9000(&instruction);

        assert_eq!(crane.supplies, expected_supplies);
    }

    #[test]
    fn test_crane_execute_9001() {
        let input = vec![
            String::from("    [D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ];

        let instruction = Instruction::from("move 3 from 2 to 3");

        let mut crane = Crane::from(&input);

        let expected_supplies = Supplies {
            stacks: vec![vec!['Z', 'N'], vec![], vec!['P', 'M', 'C', 'D']],
        };

        crane.execute_9001(&instruction);

        assert_eq!(crane.supplies, expected_supplies);
    }

    #[test]
    fn test_crane_top_supplies() {
        let input = vec![
            String::from("    [D]        "),
            String::from("[N] [C]        "),
            String::from("[Z] [M] [P]    "),
            String::from(" 1   2   3   4 "),
        ];

        let crane = Crane::from(&input);

        let expected = vec!['N', 'D', 'P', ' '];

        let result = crane.top_supplies();

        assert_eq!(result, expected);
    }
}
