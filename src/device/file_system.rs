use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::commands::{ChangeDirectory, ExecutedCommand, List};

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: u64,
}

impl From<&String> for File {
    fn from(input: &String) -> File {
        let (size, name) = input
            .split_once(' ')
            .map(|(size, name)| {
                (
                    size.parse().expect(&format!("Not a number: {}", size)),
                    name.to_string(),
                )
            })
            .expect(&format!("Couldn't Parse: {}", input));

        File { name, size }
    }
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    directories: BTreeMap<String, Rc<RefCell<Directory>>>,
    files: BTreeMap<String, File>,
}

impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            directories: BTreeMap::new(),
            files: BTreeMap::new(),
        }
    }

    fn add_sub_directory(&mut self, directory: &Rc<RefCell<Directory>>) {
        self.directories
            .insert(directory.borrow().name.clone(), Rc::clone(directory));
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }
}

#[derive(Debug, PartialEq)]
struct FileSystem {
    root_directory: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root_directory: Rc::new(RefCell::new(Directory::new("/"))),
        }
    }

    fn create_from_executed_commands(executed_commands: &[ExecutedCommand]) -> FileSystem {
        let file_system = FileSystem::new();

        let mut path: Vec<Rc<RefCell<Directory>>> = vec![];

        let mut current_directory: Rc<RefCell<Directory>> = Rc::clone(&file_system.root_directory);

        for command in executed_commands.iter() {
            match command {
                ExecutedCommand::ChangeDirectory(ChangeDirectory::To(dir_name)) => {
                    if &current_directory.borrow().name == dir_name {
                        continue;
                    }

                    path.push(Rc::clone(&current_directory));

                    let temp =
                        Rc::clone(current_directory.borrow().directories.get(dir_name).expect(
                            &format!(
                                "Current directory {} missing directory {}",
                                &current_directory.borrow().name,
                                dir_name
                            ),
                        ));

                    current_directory = temp;
                }
                ExecutedCommand::ChangeDirectory(ChangeDirectory::Out) => {
                    current_directory =
                        path.pop().unwrap_or(Rc::clone(&file_system.root_directory));
                }
                ExecutedCommand::ChangeDirectory(ChangeDirectory::Root) => {
                    path = vec![];
                    current_directory = Rc::clone(&file_system.root_directory);
                }
                ExecutedCommand::List(list) => {
                    list.directories
                        .iter()
                        .filter(|&dir_name| {
                            !current_directory
                                .borrow()
                                .directories
                                .contains_key(dir_name)
                        })
                        .for_each(|dir_name| {
                            let dir = Rc::new(RefCell::new(Directory::new(dir_name)));

                            current_directory
                                .borrow_mut()
                                .directories
                                .insert(dir_name.to_string(), dir);
                        });

                    list.files.iter().for_each(|file_info| {
                        let file = File::from(file_info);

                        current_directory
                            .borrow_mut()
                            .files
                            .insert(file.name.to_string(), file);
                    });
                }
            }
        }

        file_system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_from() {
        let input = String::from("14848514 b.txt");

        let expected = File {
            name: String::from("b.txt"),
            size: 14_848_514,
        };

        let result = File::from(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_subdirectory() {
        let mut directory = Directory::new("/");
        let sub_directory = Rc::new(RefCell::new(Directory::new("a")));

        directory.add_sub_directory(&sub_directory);

        assert_eq!(directory.directories.len(), 1);
        assert_eq!(*directory.directories.get("a").unwrap(), sub_directory);
    }

    #[test]
    fn test_add_file() {
        let mut directory = Directory::new("/");
        let file = File {
            name: String::from("b.txt"),
            size: 14_848_514,
        };

        directory.add_file(file);

        assert_eq!(directory.files.len(), 1);
        assert_eq!(
            directory.files.get("b.txt").unwrap().name,
            String::from("b.txt")
        );
        assert_eq!(directory.files.get("b.txt").unwrap().size, 14_848_514);
    }

    #[test]
    fn test_add_file_then_directory() {
        let mut directory = Directory::new("/");
        let sub_directory = Rc::new(RefCell::new(Directory::new("a")));
        let file = File {
            name: String::from("f"),
            size: 29_116,
        };

        sub_directory.borrow_mut().add_file(file);
        directory.add_sub_directory(&sub_directory);

        assert_eq!(directory.directories.len(), 1);
        assert_eq!(*directory.directories.get("a").unwrap(), sub_directory);
    }

    #[test]
    fn test_populate_from_executed_commands() {
        let commands = vec![
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Root),
            ExecutedCommand::List(List {
                directories: vec![String::from("a"), String::from("d")],
                files: vec![
                    String::from("14848514 b.txt"),
                    String::from("8504156 c.dat"),
                ],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("a"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("584 i")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Out),
        ];

        let result = FileSystem::create_from_executed_commands(&commands);
    }
}
