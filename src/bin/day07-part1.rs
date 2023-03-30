use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File as StdFile;
use std::io::{self, BufRead};
use std::rc::Rc;

use derivative::Derivative;
use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Filesystem {
    root: Rc<Directory>,
}

impl Filesystem {
    fn new() -> Self {
        Filesystem {
            root: Rc::new(Directory::new_root()),
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Directory {
    name: String,
    #[derivative(Debug = "ignore")]
    parent: Option<Rc<Directory>>,
    children: RefCell<BTreeMap<String, Rc<Entry>>>,
}

impl Directory {
    fn new_root() -> Self {
        Directory {
            name: "/".to_string(),
            parent: None,
            children: RefCell::new(BTreeMap::new()),
        }
    }

    fn new(name: String, parent: Rc<Directory>) -> Directory {
        Directory {
            name,
            parent: Some(parent),
            children: RefCell::new(BTreeMap::new()),
        }
    }

    fn get_subdirectory(&self, dir: &str) -> Rc<Directory> {
        if dir == ".." {
            self.parent
                .clone()
                .expect("expecting to have a parent directory when calling `cd ..`")
        } else {
            match self.children.borrow().get(dir) {
                Some(entry) => match &**entry {
                    Entry::Directory(directory) => Rc::clone(directory),
                    Entry::File(_) => panic!(
                        "found a file with name {} but not a directory within {}",
                        &dir, &self.name
                    ),
                },
                None => panic!(
                    "could not execute ChangeDirectory as no {} directory found within {}",
                    &dir, &self.name
                ),
            }
        }
    }
}

#[derive(Debug)]
enum Entry {
    File(Rc<File>),
    Directory(Rc<Directory>),
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Command {
    #[display("$ cd {0}")]
    ChangeDirectory(String),
    #[display("$ ls")]
    ListDirectory,
}

struct ShellSession {
    current_command: Command,
    current_working_directory: Rc<Directory>,
}

fn get_filesize(dir: Rc<Directory>, size_map: &mut Vec<u32>) -> u32 {
    let mut total = 0;
    for entry in dir.children.borrow().values() {
        total += match &**entry {
            Entry::Directory(directory) => {
                let size = get_filesize(Rc::clone(directory), size_map);
                if size <= 100000 {
                    size_map.push(size);
                }
                size
            }
            Entry::File(file) => file.size,
        }
    }
    total
}

fn main() -> Result<(), Rc<dyn Error>> {
    let filepath = env!("CARGO_MANIFEST_DIR").to_string() + "/resources/day07.txt";
    let file = StdFile::open(filepath).unwrap();

    let mut lines = io::BufReader::new(file).lines();

    // Breaking out the first line separately so that we can ensure that
    // we are starting from the root directory.
    // In addition, it allows us to set up the ShellSession without using
    // Option.

    let first = lines
        .next()
        .expect("first line missing")
        .expect("error reading file");
    if first != "$ cd /" {
        panic!("we are assuming that the first line is cding into the root directory");
    }

    let filesystem = Filesystem::new();
    let mut session = ShellSession {
        current_command: first.parse().expect("unable to parse command correctly"),
        current_working_directory: Rc::clone(&filesystem.root),
    };

    for line in lines {
        let line = line.unwrap();

        if line == "" {
            continue;
        }

        // input is a command, we should parse it and update the session
        if line.starts_with("$") {
            session.current_command = line.parse().expect("unable to parse command correctly");
            if let Command::ChangeDirectory(dir) = &session.current_command {
                let changed_directory = session.current_working_directory.get_subdirectory(dir);
                session.current_working_directory = Rc::clone(&changed_directory);
            }

        // regular line of input, not a command
        } else {
            if let Command::ListDirectory = &session.current_command {
                let (name, entry) = if (&line).starts_with("dir ") {
                    let mut split = line.split_ascii_whitespace();
                    split.next().unwrap();
                    let name = split.next().unwrap();
                    let parent = Rc::clone(&session.current_working_directory);
                    let directory = Rc::new(Directory::new(name.to_string(), parent));
                    let entry = Rc::new(Entry::Directory(directory));
                    (name, entry)
                } else {
                    let mut split = line.split_ascii_whitespace();
                    let size: u32 = split.next().unwrap().parse().unwrap();
                    let name = split.next().unwrap();
                    let file = Rc::new(File {
                        name: name.to_string(),
                        size,
                    });
                    let entry = Rc::new(Entry::File(file));
                    (name, entry)
                };

                session
                    .current_working_directory
                    .children
                    .borrow_mut()
                    .insert(name.to_string(), entry);
            }
        }
    }

    // now iterate down the tree and get size of each directory
    let mut directory_sizes = vec![];
    let total = get_filesize(Rc::clone(&filesystem.root), &mut directory_sizes);
    if total <= 100000 {
        directory_sizes.push(total);
    }
    println!("{}", &directory_sizes.into_iter().sum::<u32>());

    Ok(())
}
