use std::{
    io::{stdin, Read},
    num::ParseIntError,
    str::FromStr, cmp::min,
};

#[derive(Debug)]
struct File {
    id: usize,
    parent: usize,
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Dir {
    id: usize,
    parent: usize,
    name: String,
    children_ids: Vec<usize>,
}
#[derive(Debug)]
enum FileKind {
    File(File),
    Dir(Dir),
}

impl FromStr for FileKind {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.clone().split_once(" ").unwrap();
        if first == "dir" {
            Ok(FileKind::Dir(Dir {
                id: 0,
                name: String::from_str(second).unwrap(),
                children_ids: Vec::new(),
                parent: 0,
            }))
        } else {
            let size = first.parse()?;
            Ok(FileKind::File(File {
                id: 0,
                parent: 0,
                name: String::from_str(second).unwrap(),
                size,
            }))
        }
    }
}

impl FileKind {
    fn dir(&mut self) -> &mut Dir {
        if let FileKind::Dir(d) = self {
            d
        } else {
            panic!("Not a Dir")
        }
    }

    fn dir_ref(&self) -> &Dir {
        if let FileKind::Dir(d) = self {
            d
        } else {
            panic!("Not a Dir")
        }
    }
}

struct Store {
    contents: Vec<FileKind>,
}

impl Store {
    fn new_dir(&mut self, name: &str, parent: usize) -> usize {
        let ret = FileKind::Dir(Dir {
            id: self.contents.len(),
            name: String::from_str(name).unwrap(),
            parent,
            children_ids: Vec::new(),
        });
        let new_id = self.contents.len();
        if parent != new_id {
            self.contents[parent].dir().children_ids.push(new_id);
            println!(
                "Adding dir: {:?}\nparent:{:?}",
                ret, self.contents[parent]
            )
        }
        self.contents.push(ret);
        new_id
    }

    fn new_file(&mut self, name: &str, size: usize, parent: usize) -> usize {
        let ret = FileKind::File(File {
            id: self.contents.len(),
            name: String::from_str(name).unwrap(),
            parent,
            size,
        });
        let new_id = self.contents.len();
        if parent != new_id {
            self.contents[parent].dir().children_ids.push(new_id);
            println!(
                "Adding file: {:?}\nparent:{:?}",
                ret, self.contents[parent]
            )
        }
        self.contents.push(ret);
        new_id
    }

    fn get_by_id(&mut self, id: usize) -> &mut FileKind {
        &mut self.contents[id]
    }

    fn get_ref_by_id(&self, id: usize) -> &FileKind {
        &self.contents[id]
    }
}

fn dfs(cur: usize, store: &Store) -> (usize, usize) {
    let node = store.get_ref_by_id(cur);
    if let FileKind::File(f) = node {
        return (0, f.size);
    }
    let children = &node.dir_ref().children_ids;
    let mut my_subtree_size = 0;
    let mut my_subtree_ans = 0;
    for child in children {
        let (subtree_ans, subtree_size) = dfs(*child, store);
        my_subtree_ans += subtree_ans;
        my_subtree_size += subtree_size;
    }
    if my_subtree_size <= 100000 {
        my_subtree_ans += my_subtree_size;
    }
    (my_subtree_ans, my_subtree_size)
}

fn dfs2(cur: usize, store: &Store, target: usize) -> (usize, usize) {
    let node = store.get_ref_by_id(cur);
    if let FileKind::File(f) = node {
        return (70000001, f.size)
    }
    let children = &node.dir_ref().children_ids;
    let mut my_subtree_size = 0;
    let mut my_subtree_ans = 70000001;
    for child in children {
        let (subtree_ans, subtree_size) = dfs2(*child, store, target);
        my_subtree_ans = min(my_subtree_ans, subtree_ans);
        my_subtree_size += subtree_size;
    }
    if my_subtree_size >= target {
        my_subtree_ans = min(my_subtree_ans, my_subtree_size);
    }
    (my_subtree_ans, my_subtree_size)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Can't read from stdin");
    let commands: Vec<&str> = input.split("\n$").collect();
    // remove the first command as it's assumed to be "cd /".
    let iter = commands.iter().skip(1);
    let mut store = crate::Store {
        contents: Vec::new(),
    };
    let mut pwd = store.new_dir("", 0);

    for command in iter {
        if command.is_empty() {
            break;
        }
        println!("Parsing command {:?}", command);
        let mut lines = command.split("\n").map(|s| s.trim());
        let line0 = lines.next().unwrap();
        println!("Command is: {:?}", line0);
        if line0 == "ls" {
            for line in lines {
                if line.is_empty() {
                    break;
                }
                let child: FileKind = line.parse().unwrap();
                _ = match child {
                    FileKind::File(f) => store.new_file(&f.name, f.size, pwd),
                    FileKind::Dir(d) => store.new_dir(&d.name, pwd),
                }
            }
        } else {
            // It's a cd command
            let (_, dir_name) = line0.split_once(" ").unwrap();
            if dir_name == ".." {
                println!("Going to parent dir!");
                pwd = store.get_by_id(pwd).dir().parent;
                continue;
            }
            // Just clone it to avoid mutable borrow :(
            let children = &store.get_by_id(pwd).dir().children_ids.clone();
            for child in children {
                if let FileKind::Dir(d) = store.get_by_id(*child) {
                    println!("Found name: {:?}", d);
                    if d.name == dir_name {
                        pwd = d.id;
                        println!("Found next child: {:?}", d);
                        break;
                    }
                }
            }
        }
    };
    let ans1 = dfs(0, &store);
    // println!("{}", ans1.0);
    let total_space = 70000000 as usize;
    let remaining_space = total_space - ans1.1;
    let required_space = 30000000;
    let ans2 = dfs2(0, &store, required_space - remaining_space);
    println!("{}", ans2.0);
}
