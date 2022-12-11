use std::rc::Rc;
use aoctools::{parse, read_file};
use filesystem::{Directory};
use crate::filesystem::File;

fn main() {

    let root_dir = Directory::root("/");
    let mut current_dir = Rc::clone(&root_dir);

    for line_res in read_file("input.txt") {
        match line_res {
            Ok(line) => {
                current_dir = process_line(&line, Rc::clone(&current_dir), Rc::clone(&root_dir));
            },
            Err(e) => panic!("Problem when reading a file: {:?}", e),
        }
    }

    let part1: usize = get_all_dirs(&root_dir).into_iter().filter(|d| d.size() <= 100000).map(|d| d.size()).sum();
    println!("Part one: {}", part1);

    let unused = 70000000 - root_dir.size();
    let needed = 30000000 - unused;
    let mut sorted: Vec<usize> = get_all_dirs(&root_dir).into_iter().map(|d| d.size()).filter(|s| s >= &needed).collect::<Vec<usize>>();
    sorted.sort();
    println!("Part two: {:?}", sorted[0]);

}

fn get_all_dirs(dir: &Rc<Directory>) -> Vec<Rc<Directory>> {
    let mut dirs = Vec::new();
    for x in dir.get_directories().iter() {
        // println!("dir: {}", x.get_name());
        dirs.push(Rc::clone(x));
        let dirs_inside = get_all_dirs(&Rc::clone(x));
        dirs.extend(dirs_inside);
    }
    dirs
}

fn process_line(line: &str, current_dir: Rc<Directory>, root_dir: Rc<Directory>) -> Rc<Directory> {
    if line.starts_with("$ cd ") {
        let next_dir_name = line.split("cd ").into_iter().collect::<Vec<&str>>()[1];
        if next_dir_name.trim() == ".." {
            match current_dir.get_parent() {
                Some(rc_dir) => {
                    return rc_dir;
                },
                None => panic!("cd .. for root directory"),
            }
        } else if next_dir_name.trim() == "/" {
            return root_dir;
        } else {
            match current_dir.get_directory(next_dir_name.trim()) {
                Some(rc_dir) => {
                    return rc_dir;
                },
                None => panic!("cd into non existing directory: {}", line)
            }
        }
    } else if line.starts_with("dir ") {
        let new_dir_name = &line[4..];
        let new_dir = Directory::empty(new_dir_name, Rc::downgrade(&current_dir));
        current_dir.add_directory(&new_dir);
    } else if line.starts_with("$ ls") {

    } else {
        let split = line.split(" ").into_iter().collect::<Vec<&str>>();
        let (file_size, file_name) = (parse(split[0]), split[1]);
        let new_file = File::new(file_name, file_size as usize);
        current_dir.add_file(&new_file);
    }
    return Rc::clone(&current_dir);
}

pub mod filesystem {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct File {
        name: String,
        size: usize,
    }

    impl File {
        pub fn new(name: &str, size: usize) -> Rc<File> {
            Rc::new(File { name: name.to_string(), size })
        }
        pub fn to_string(&self) -> String {
            let mut res = "F[name: ".to_string();
            res.push_str(&self.name);
            res.push_str(", ");
            res.push_str(&self.size.to_string());
            res.push_str("]");
            res
        }
    }

    #[derive(Debug)]
    pub struct Directory {
        name: String,
        this: RefCell<Weak<Directory>>,
        parent: RefCell<Weak<Directory>>,
        files: RefCell<HashMap<String, Rc<File>>>,
        directories: RefCell<HashMap<String, Rc<Directory>>>,
    }

    impl Directory {

        pub fn root(name: &str) -> Rc<Directory> {
            let d = Rc::new(Directory {
                name: name.to_string(),
                this: RefCell::new(Weak::new()),
                parent: RefCell::new(Weak::new()),
                files: RefCell::new(HashMap::new()),
                directories: RefCell::new(HashMap::new()),
            });
            *(d.this.borrow_mut()) = Rc::downgrade(&d);
            d
        }

        pub fn empty(name: &str, parent: Weak<Directory>) -> Rc<Directory> {
            let dir = Directory::root(name);
            *(dir.parent.borrow_mut()) = parent;
            dir
        }

        pub fn add_directory(&self, dir: &Rc<Directory>) -> () {
            let cl = Rc::clone(&dir);
            let n = (&cl.name).to_string();
            self.directories.borrow_mut().insert(n, cl);
            if let Some(rc_dir) = self.this.borrow().upgrade() {
                *(dir.parent.borrow_mut()) = Rc::downgrade(&rc_dir);
            }
        }

        pub fn add_file(&self, file: &Rc<File>) -> () {
            let cl = Rc::clone(&file);
            let n = (&cl.name).to_string();
            self.files.borrow_mut().insert(n, cl);
        }

        pub fn size(&self) -> usize {
            let mut res = 0;
            for x in self.directories.borrow().iter() {
                res += x.1.size();
            }
            for x in self.files.borrow().iter() {
                res += x.1.size;
            }
            res
        }

        pub fn get_parent(&self) -> Option<Rc<Directory>> {
            self.parent.borrow().upgrade()
        }

        pub fn get_name(&self) -> String {
            self.name.to_string()
        }

        pub fn get_directory(&self, name: &str) -> Option<Rc<Directory>> {
            self.directories.borrow().get(name).map(|rc_d| Rc::clone(rc_d))
        }

        pub fn get_directories(&self) -> Vec<Rc<Directory>> {
            let mut res = Vec::new();
            for (_, dir) in self.directories.borrow().iter() {
                res.push(Rc::clone(dir));
            }
            res
        }

        pub fn to_string(&self) -> String {
            let mut res = "D[name: ".to_string();
            res.push_str(&self.name);
            res.push_str(", parent: ");
            match self.parent.borrow().upgrade() {
                Some(dir) => res.push_str(dir.name.as_str()),
                None => res.push_str("none")
            }
            res.push_str(", content: [");
            let dirs = self.directories.borrow();
            let mut c = 0;
            for x in dirs.iter() {
                res.push_str(x.1.to_string().as_str());
                c += 1;
                if c < dirs.len() {
                    res.push_str(", ");
                }
            }
            res.push_str("],[");
            let files = self.files.borrow();
            let mut c = 0;
            for x in files.iter() {
                res.push_str(x.1.to_string().as_str());
                c += 1;
                if c < dirs.len() {
                    res.push_str(", ");
                }
            }
            res.push_str("]]");
            res
        }
    }

}
