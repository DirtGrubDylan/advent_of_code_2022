#[derive(Debug, PartialEq)]
pub enum ExecutedCommand {
    ChangeDirectory(ChangeDirectory),
    List(List),
}

impl ExecutedCommand {
    pub fn extract_commands(input: &[String]) -> Vec<ExecutedCommand> {
        let mut reversed_input: Vec<String> = input.iter().rev().cloned().collect();

        reversed_input
            .split_inclusive_mut(|line| line.starts_with("$"))
            .map(|split| {
                split.reverse();

                ExecutedCommand::from(split.as_ref())
            })
            .rev()
            .collect()
    }
}

impl From<&[String]> for ExecutedCommand {
    fn from(input: &[String]) -> ExecutedCommand {
        let executed_command = &input[0];

        match executed_command {
            s if s.starts_with("$ cd") => {
                ExecutedCommand::ChangeDirectory(ChangeDirectory::from(executed_command))
            }
            s if s.starts_with("$ ls") => ExecutedCommand::List(List::from(input)),
            _ => panic!("Unknown command: {}", executed_command),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangeDirectory {
    To(String),
    Out,
    Root,
}

impl From<&String> for ChangeDirectory {
    fn from(input: &String) -> ChangeDirectory {
        match input.as_str() {
            "$ cd .." => ChangeDirectory::Out,
            "$ cd /" => ChangeDirectory::Root,
            _ => ChangeDirectory::To(input[5..].to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct List {
    pub directories: Vec<String>,
    pub files: Vec<String>,
}

impl From<&[String]> for List {
    fn from(input: &[String]) -> List {
        let mut directories = vec![];
        let mut files = vec![];

        for line in input.iter() {
            match line {
                s if s.starts_with("$") => (),
                s if s.starts_with("dir") => directories.push(line[4..].to_string()),
                _ => files.push(line.to_string()),
            }
        }

        List { directories, files }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_change_directory_to() {
        let input = [String::from("$ cd x")];

        let expected = ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("x")));

        let result = ExecutedCommand::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_command_change_directory_out() {
        let input = [String::from("$ cd ..")];

        let expected = ExecutedCommand::ChangeDirectory(ChangeDirectory::Out);

        let result = ExecutedCommand::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_command_change_directory_root() {
        let input = [String::from("$ cd /")];

        let expected = ExecutedCommand::ChangeDirectory(ChangeDirectory::Root);

        let result = ExecutedCommand::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_command_list() {
        let input = [
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
        ];

        let expected = ExecutedCommand::List(List {
            directories: vec![String::from("a"), String::from("d")],
            files: vec![
                String::from("14848514 b.txt"),
                String::from("8504156 c.dat"),
            ],
        });

        let result = ExecutedCommand::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_commands() {
        let input = vec![
            String::from("$ cd /"),
            String::from("$ ls"),
            String::from("dir a"),
            String::from("14848514 b.txt"),
            String::from("8504156 c.dat"),
            String::from("dir d"),
            String::from("$ cd e"),
            String::from("$ ls"),
            String::from("584 i"),
            String::from("$ cd .."),
        ];

        let expected = vec![
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Root),
            ExecutedCommand::List(List {
                directories: vec![String::from("a"), String::from("d")],
                files: vec![
                    String::from("14848514 b.txt"),
                    String::from("8504156 c.dat"),
                ],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("e"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("584 i")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Out),
        ];

        let result: Vec<ExecutedCommand> = ExecutedCommand::extract_commands(&input);

        assert_eq!(result, expected);
    }
}
