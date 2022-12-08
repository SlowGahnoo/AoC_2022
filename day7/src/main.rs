use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct DirectoryTree {
    root: Rc<RefCell<Node>>,
}

#[derive(Debug)]
struct Node {
    name: String,
    parent: Option<Weak<RefCell<Node>>>,
    directories: Vec<Rc<RefCell<Node>>>,
    files: Vec<File>,
}

impl DirectoryTree {
    fn new() -> Self {
        DirectoryTree {
            root: Rc::new(RefCell::new(Node::new("/"))),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            directories: vec![],
            parent: None,
            files: vec![],
        }
    }

    fn add_dir(parent: &Rc<RefCell<Node>>, name: &str) {
        let new_node = Rc::new(RefCell::new(Node::new(name)));
        new_node.borrow_mut().parent = Some(Rc::downgrade(&parent));
        parent.borrow_mut().directories.push(new_node);
    }

    fn get_parent(dir: &Node) -> Option<Rc<RefCell<Node>>> {
        match &dir.parent {
            Some(parent) => parent.upgrade(),
            None => None,
        }
    }
    fn get_size(dir: &Node) -> usize {
        let mut size = 0;
        for f in dir.files.iter() {
            size += f.size;
        }
        for d in dir.directories.iter() {
            size += Self::get_size(&d.borrow());
        }
        size
    }

    fn get_full_path(dir: &Node) -> String {
        let mut path = dir.name.to_string();
        match dir.parent.as_ref() {
            Some(p) => path = Self::get_full_path(&p.upgrade().unwrap().borrow()) + &path + "/",
            None => (),
        }
        path
    }
}

/// Change directory
fn cd(path: &str, tree_ptr: &mut Rc<RefCell<Node>>, filetree: &DirectoryTree) {
    match path {
        "/" => *tree_ptr = Rc::clone(&filetree.root),
        ".." => {
            let p = match Node::get_parent(&tree_ptr.borrow()) {
                Some(directory) => Rc::clone(&directory),
                None => Rc::clone(&filetree.root),
            };
            *tree_ptr = p;
        }
        dir => {
            let mut val: Rc<RefCell<Node>> = Rc::clone(&filetree.root);
            for d in tree_ptr.borrow_mut().directories.iter() {
                if d.borrow_mut().name == dir {
                    val = d.clone();
                }
            }
            *tree_ptr = Rc::clone(&val);
        }
    }
}

/// Make directory
fn mkdir(name: &str, tree_ptr: &mut Rc<RefCell<Node>>) {
    Node::add_dir(tree_ptr, name);
}

/// Create file
fn touch(name: &str, size: usize, tree_ptr: &mut Rc<RefCell<Node>>) {
    tree_ptr.borrow_mut().files.push(File::new(name, size));
}

fn map_sizes(dir: &Node, h: &mut HashMap<String, usize>) {
    let size = Node::get_size(dir);
    h.insert(Node::get_full_path(dir), size);
    for d in dir.directories.iter() {
        map_sizes(&d.borrow(), h);
    }
}

fn initialise_filetree(input: &str, filetree: &DirectoryTree) {
    let mut tree_ptr: Rc<RefCell<Node>> = Rc::clone(&filetree.root);

    let mut lines = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect::<VecDeque<Vec<&str>>>();

    while let Some(command) = lines.pop_front() {
        match command[1] {
            "cd" => {
                let path = command[2];
                cd(path, &mut tree_ptr, &filetree);
            }
            "ls" => {
                while let Some(output) = lines.pop_front() {
                    if output[0] == "$" {
                        lines.push_front(output);
                        break;
                    }
                    if output[0] == "dir" {
                        let name = output[1];
                        mkdir(name, &mut tree_ptr);
                    } else if output[0].chars().all(char::is_numeric) {
                        let name = output[1];
                        let size = output[0].parse::<usize>().unwrap();
                        touch(name, size, &mut tree_ptr);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

fn part1(dir_map: &HashMap<String, usize>) -> usize {
    dir_map
        .values()
        .filter(|n| *n <= &(100_000 as usize))
        .sum::<usize>()
}

fn part2(dir_map: &HashMap<String, usize>) -> usize {
    *dir_map
        .values()
        .filter(|n| *n >= &(30_000_000 - (70_000_000 - dir_map.get("/").unwrap())))
        .min()
        .unwrap()
}

fn main() {
    let filetree = DirectoryTree::new();

    let input = fs::read_to_string("input.txt").unwrap();
    initialise_filetree(&input, &filetree);

    let mut dir_map: HashMap<String, usize> = HashMap::new();
    map_sizes(&filetree.root.borrow(), &mut dir_map);

    println!("Part 1: {}", part1(&dir_map));
    println!("Part 2: {}", part2(&dir_map));
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part1_test() {
        let filetree = DirectoryTree::new();
        let input = fs::read_to_string("test.txt").unwrap();
        initialise_filetree(&input, &filetree);
        let mut dir_map: HashMap<String, usize> = HashMap::new();
        map_sizes(&filetree.root.borrow(), &mut dir_map);
        assert_eq!(95437, part1(&dir_map));
    }

    #[test]
    fn part2_test() {
        let filetree = DirectoryTree::new();
        let input = fs::read_to_string("test.txt").unwrap();
        initialise_filetree(&input, &filetree);
        let mut dir_map: HashMap<String, usize> = HashMap::new();
        map_sizes(&filetree.root.borrow(), &mut dir_map);
        assert_eq!(24933642, part2(&dir_map));
    }
}
