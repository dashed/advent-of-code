// https://adventofcode.com/2022/day/7

use std::sync::RwLock;
use std::sync::{Arc, Weak};

#[derive(Debug, Clone)]
enum Command {
    List,
    ChangeDirectory(String),
}

impl Command {
    fn parse(input: String) -> Self {
        let input: Vec<String> = input
            .split_whitespace()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(input[0], "$");

        match input[1].as_str() {
            "ls" => {
                assert_eq!(input.len(), 2);
                Command::List
            }
            "cd" => {
                assert_eq!(input.len(), 3);
                Command::ChangeDirectory(input[2].to_string())
            }
            _ => {
                unreachable!();
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Content {
    File(File),
    Directory(Arc<RwLock<Directory>>),
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    contents: Vec<Content>,
    parent_dir: Option<Weak<RwLock<Directory>>>,
}

impl Directory {
    fn create_filesystem() -> Self {
        Directory {
            name: "/".to_string(),
            contents: vec![],
            parent_dir: None,
        }
    }

    fn create(name: String, parent_dir: Weak<RwLock<Directory>>) -> Self {
        Directory {
            name,
            contents: vec![],
            parent_dir: Some(parent_dir),
        }
    }

    fn add_content(&mut self, content: Content) {
        self.contents.push(content);
    }

    fn get_total_size(&self) -> usize {
        self.contents
            .iter()
            .map(|content| match content {
                Content::Directory(dir) => dir.read().unwrap().get_total_size(),
                Content::File(file) => file.size,
            })
            .sum()
    }

    fn _print_debug(current_directory: Arc<RwLock<Directory>>, level: usize) {
        let dir = current_directory.read().unwrap();
        let dir_name = dir.name.clone();
        let output = format!("{} (dir, size={})", dir_name, dir.get_total_size());
        let padding = " ".repeat(level * 3);
        println!("{}- {}", padding, output);

        for content in dir.contents.iter() {
            match content {
                Content::Directory(dir) => {
                    Self::_print_debug(dir.clone(), level + 1);
                }
                Content::File(file) => {
                    let padding = " ".repeat((level + 1) * 3);
                    let file_name = file.name.clone();
                    let output = format!("{} (file, size={})", file_name, file.size);
                    println!("{}- {}", padding, output);
                }
            }
        }
    }

    fn print_debug(directory: Arc<RwLock<Directory>>) {
        Self::_print_debug(directory, 0);
    }
}

fn make_root_filesytem(input_string: String) -> Arc<RwLock<Directory>> {
    let lines: Vec<String> = input_string
        .trim()
        .split('\n')
        .map(|x| -> String { x.to_string() })
        .collect();

    let root_filesytem = Arc::new(RwLock::new(Directory::create_filesystem()));

    let mut current_directory = root_filesytem.clone();

    let mut parse_input_for_command = None;

    for line in lines {
        let line = line.trim().to_string();
        if line.starts_with('$') {
            parse_input_for_command = None;

            // parse command
            let parsed_command = Command::parse(line);

            match parsed_command {
                Command::ChangeDirectory(directory_name) => match directory_name.as_str() {
                    "/" => {
                        current_directory = root_filesytem.clone();
                        continue;
                    }
                    ".." => {
                        let parent_dir = { current_directory.read().unwrap().parent_dir.clone() };
                        if let Some(parent_dir) = parent_dir {
                            current_directory = parent_dir.upgrade().unwrap().clone();
                            continue;
                        }

                        unreachable!();
                    }
                    _ => {
                        let needle = current_directory
                            .read()
                            .unwrap()
                            .contents
                            .iter()
                            .find(|x| match x {
                                Content::Directory(dir) => {
                                    dir.read().unwrap().name == directory_name
                                }
                                _ => false,
                            })
                            .unwrap()
                            .clone();

                        assert!(matches!(needle, Content::Directory(_)));

                        match needle {
                            Content::Directory(directory) => {
                                current_directory = directory;
                            }
                            _ => {
                                unreachable!();
                            }
                        }
                        continue;
                    }
                },
                Command::List => parse_input_for_command = Some(parsed_command.clone()),
            }

            continue;
        }

        assert!(matches!(parse_input_for_command, Some(Command::List)));

        let inputs: Vec<String> = line
            .split_whitespace()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();

        assert_eq!(inputs.len(), 2);

        if inputs[0].as_str() == "dir" {
            let directory =
                Directory::create(inputs[1].clone(), Arc::downgrade(&current_directory));
            let directory = Arc::new(RwLock::new(directory));

            current_directory
                .write()
                .unwrap()
                .add_content(Content::Directory(directory.clone()));

            continue;
        }

        let file_size: usize = inputs[0].parse().unwrap();
        let file = File {
            name: inputs[1].clone(),
            size: file_size,
        };
        current_directory
            .write()
            .unwrap()
            .add_content(Content::File(file));
    }

    root_filesytem
}

fn part_1(input_string: String) -> usize {
    let root_filesytem = make_root_filesytem(input_string);

    Directory::print_debug(root_filesytem.clone());

    let mut to_visit = vec![root_filesytem];

    let mut sum_of_sizes = 0;
    loop {
        if to_visit.is_empty() {
            break;
        }

        let current_directory = to_visit.pop().unwrap();

        let mut directories: Vec<_> = current_directory
            .read()
            .unwrap()
            .contents
            .iter()
            .filter(|x| matches!(x, Content::Directory(_)))
            .map(|x| match x.clone() {
                Content::Directory(dir) => dir,
                _ => unreachable!(),
            })
            .collect();
        to_visit.append(&mut directories);

        let total_size = current_directory.read().unwrap().get_total_size();
        if total_size <= 100000 {
            sum_of_sizes += total_size;
        }
    }

    sum_of_sizes
}

fn part_2(input_string: String) -> usize {
    let root_filesytem = make_root_filesytem(input_string);

    let mut to_visit = vec![root_filesytem.clone()];

    let root_dir_size = root_filesytem.read().unwrap().get_total_size();
    let total_disk_size_available: usize = 70_000_000;
    let current_unused_space = total_disk_size_available - root_dir_size;
    let required_unused_space: usize = 30_000_000;

    assert!(root_dir_size <= total_disk_size_available);

    let mut candidate_directory_size = root_filesytem.read().unwrap().get_total_size();
    loop {
        if to_visit.is_empty() {
            break;
        }

        let current_directory = to_visit.pop().unwrap();

        let mut directories: Vec<_> = current_directory
            .read()
            .unwrap()
            .contents
            .iter()
            .filter(|x| matches!(x, Content::Directory(_)))
            .map(|x| match x.clone() {
                Content::Directory(dir) => dir,
                _ => unreachable!(),
            })
            .collect();
        to_visit.append(&mut directories);

        let total_size = current_directory.read().unwrap().get_total_size();
        if total_size < candidate_directory_size {
            let candidate_unused_space = current_unused_space + candidate_directory_size;
            let next_unused_space = current_unused_space + total_size;
            if next_unused_space < candidate_unused_space
                && next_unused_space > required_unused_space
            {
                candidate_directory_size = total_size;
            }
        }
    }

    assert!((current_unused_space + candidate_directory_size) > required_unused_space);
    candidate_directory_size
}

fn main() {
    let input_string = include_str!("input.txt");

    let part_1_answer = part_1(input_string.to_string());
    println!("Part 1: {}", part_1_answer);
    assert_eq!(part_1_answer, 1501149);

    let part_2_answer = part_2(input_string.to_string());
    println!("Part 2: {}", part_2_answer);
    assert_eq!(part_2_answer, 10096985);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let input_string = r###"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"###
        .to_string();

        assert_eq!(part_1(input_string.clone()), 95437);
        assert_eq!(part_2(input_string), 24933642);
    }
}
