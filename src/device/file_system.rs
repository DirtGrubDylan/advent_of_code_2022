use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
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

    fn get_size(&self) -> u64 {
        let total_sub_directory_size = self
            .directories
            .values()
            .fold(0, |acc, sub_dir| acc + sub_dir.borrow().get_size());

        let total_files_size = self.files.values().fold(0, |acc, file| acc + file.size);

        total_sub_directory_size + total_files_size
    }

    fn add_sub_directory(&mut self, directory: &Rc<RefCell<Directory>>) {
        self.directories
            .entry(directory.borrow().name.to_string())
            .or_insert(Rc::clone(directory));
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.clone(), file);
    }

    fn populate_from_executed_list_command(&mut self, list: &List) {
        list.directories.iter().for_each(|dir_name| {
            self.add_sub_directory(&Rc::new(RefCell::new(Directory::new(dir_name))));
        });

        list.files
            .iter()
            .for_each(|file_info| self.add_file(File::from(file_info)));
    }
}

#[derive(Debug, PartialEq)]
pub struct FileSystem {
    root_directory: Rc<RefCell<Directory>>,
}

impl FileSystem {
    pub fn get_size(&self) -> u64 {
        self.root_directory.borrow().get_size()
    }

    pub fn directory_sizes_while<P>(&self, predicate: P) -> HashMap<String, u64>
    where
        P: Fn(u64) -> bool,
    {
        let mut sizes = HashMap::new();

        let mut path: Vec<Rc<RefCell<Directory>>> = vec![Rc::clone(&self.root_directory)];

        while let Some(current_directory) = path.pop() {
            let current_directory_size = current_directory.borrow().get_size();

            if predicate(current_directory_size) {
                sizes.insert(
                    current_directory.borrow().name.to_string(),
                    current_directory_size,
                );
            }

            current_directory
                .borrow()
                .directories
                .values()
                .for_each(|rc| path.push(Rc::clone(rc)));
        }

        sizes
    }

    pub fn sum_of_directory_sizes_while<P>(&self, predicate: P) -> u64
    where
        P: Fn(u64) -> bool,
    {
        let mut sum = 0;

        let mut path: Vec<Rc<RefCell<Directory>>> = vec![Rc::clone(&self.root_directory)];

        while let Some(current_directory) = path.pop() {
            let current_directory_size = current_directory.borrow().get_size();

            if predicate(current_directory_size) {
                sum += current_directory_size
            }

            current_directory
                .borrow()
                .directories
                .values()
                .for_each(|rc| path.push(Rc::clone(rc)));
        }

        sum
    }
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            root_directory: Rc::new(RefCell::new(Directory::new("/"))),
        }
    }

    pub fn create_from_executed_commands(executed_commands: &[ExecutedCommand]) -> FileSystem {
        let file_system = FileSystem::new();

        let mut path: Vec<Rc<RefCell<Directory>>> = vec![];

        let mut current_directory: Rc<RefCell<Directory>> = Rc::clone(&file_system.root_directory);

        for command in executed_commands.iter() {
            match command {
                ExecutedCommand::ChangeDirectory(ChangeDirectory::To(dir_name)) => {
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
                    current_directory
                        .borrow_mut()
                        .populate_from_executed_list_command(list);
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
    fn test_populate_from_executed_list_command() {
        let mut directory = Directory::new("/");
        let list = List::from([String::from("dir a"), String::from("29116 f")].as_slice());

        let sub_directory = Rc::new(RefCell::new(Directory::new("a")));
        let file = File {
            name: String::from("f"),
            size: 29_116,
        };

        directory.populate_from_executed_list_command(&list);

        assert_eq!(directory.directories.len(), 1);
        assert_eq!(*directory.directories.get("a").unwrap(), sub_directory);
        assert_eq!(*directory.files.get("f").unwrap(), file);
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

        let mut expected_directory = Directory::new("/");
        let sub_directory_a = Rc::new(RefCell::new(Directory::new("a")));
        let sub_directory_d = Rc::new(RefCell::new(Directory::new("d")));
        let file_b = File {
            name: String::from("b.txt"),
            size: 14_848_514,
        };
        let file_c = File {
            name: String::from("c.dat"),
            size: 8_504_156,
        };
        let file_i = File {
            name: String::from("i"),
            size: 584,
        };

        sub_directory_a.borrow_mut().add_file(file_i);
        expected_directory.add_sub_directory(&sub_directory_a);
        expected_directory.add_sub_directory(&sub_directory_d);
        expected_directory.add_file(file_b);
        expected_directory.add_file(file_c);

        let result = FileSystem::create_from_executed_commands(&commands);

        assert_eq!(*result.root_directory, RefCell::new(expected_directory));
    }

    #[test]
    fn test_directory_system_size() {
        let mut directory = Directory::new("a");
        let sub_directory_d = Rc::new(RefCell::new(Directory::new("d")));
        let file_b = File {
            name: String::from("b.txt"),
            size: 14_848_514,
        };
        let file_c = File {
            name: String::from("c.dat"),
            size: 8_504_156,
        };

        directory.add_sub_directory(&sub_directory_d);
        directory.add_file(file_b);
        directory.add_file(file_c);

        let expected = 23_352_670;

        let result = directory.get_size();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_system_size() {
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

        let file_system = FileSystem::create_from_executed_commands(&commands);

        let expected = 23_353_254;

        let result = file_system.get_size();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_system_directory_sizes_while() {
        let commands = vec![
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Root),
            ExecutedCommand::List(List {
                directories: vec![String::from("a"), String::from("d")],
                files: vec![String::from("14848514 b.txt")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("a"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("584 i")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Out),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("d"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("8504156 c.dat")],
            }),
        ];

        let file_system = FileSystem::create_from_executed_commands(&commands);

        let expected = HashMap::from([(String::from("a"), 584), (String::from("d"), 8_504_156)]);

        let result = file_system.directory_sizes_while(|size| size <= 9_000_000);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_system_sum_of_directory_sizes_while() {
        let commands = vec![
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Root),
            ExecutedCommand::List(List {
                directories: vec![String::from("a"), String::from("d")],
                files: vec![String::from("14848514 b.txt")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("a"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("584 i")],
            }),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::Out),
            ExecutedCommand::ChangeDirectory(ChangeDirectory::To(String::from("d"))),
            ExecutedCommand::List(List {
                directories: vec![],
                files: vec![String::from("8504156 c.dat")],
            }),
        ];

        let file_system = FileSystem::create_from_executed_commands(&commands);

        let expected = 8_504_740;

        let result = file_system.sum_of_directory_sizes_while(|size| size <= 9_000_000);

        assert_eq!(result, expected);
    }
}
