use std::str::FromStr;
use std::{io::*};

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n: usize = sc.next();
    let mut bst = BST::new();

    for _ in 0..n {
        let ord: String = sc.next();


        match ord.as_str() {
            "print" => {
                bst.print();
            }
            c => {
                let val: isize = sc.next();
                match c {
                    "insert" => bst.insert(val),
                    "find" => bst.find(&val),
                    "delete" => bst.delete(&val),
                    _ => panic!("command error")
                }
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    key: isize,
    left: BST,
    right: BST,
}

impl Node {
    pub fn new(key: isize) -> Self {
        Node {
            key,
            left: BST::new(),
            right: BST::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        Self { root: None, }
    }

    pub fn insert(&mut self, val: isize) {
        let mut current = self;
        while let Some(ref mut node) = current.root {
            if val < node.key {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        current.root = Some(Box::new(Node::new(val)));
    }

    fn find(&self, value: &isize) {
        let mut current = self;
        let mut res = false;
        while let Some(node) = &current.root {
            if *value == node.key {
                res = true;
                break;
            }
            else if *value < node.key {
                current = &current.root.as_ref().unwrap().left;
            } else {
                current = &current.root.as_ref().unwrap().right;
            }
        }
        println!("{}", if res {"yes"} else {"no"});
    }

    fn extract_min(&mut self) -> Option<isize> {
        let mut node = None;
        if self.root.is_some() {
            let mut current = self;
            while current.root.as_ref().unwrap().left.root.is_some() {
                current = &mut current.root.as_mut().unwrap().left;
            }
            let temp = current.root.take().unwrap();
            node = Some(temp.key);
            current.root = temp.right.root
        }
        node
    }

    fn delete(&mut self, value: &isize) {
        let mut current = self;
        while let Some(ref mut node) = current.root {
            if *value < node.key {
                current = &mut current.root.as_mut().unwrap().left;
            } else if *value > node.key {
                current = &mut current.root.as_mut().unwrap().right;
            } else {
                match (node.left.root.as_mut(), node.right.root.as_mut()) {
                    (None, None) => current.root = None,
                    (None, Some(_)) => current.root = node.right.root.take(),
                    (Some(_), None) => current.root = node.left.root.take(),
                    (Some(_), Some(_)) => {
                        current.root.as_mut().unwrap().key = node.right.extract_min().unwrap();
                    }
                }
            }
        }
    }

    fn in_parse(&self, res: &mut Vec<String>) {
        self.root.as_ref().map(|x| x.left.in_parse(res));
        if let Some(node) = self.root.as_ref() {
            res.push(node.key.to_string());
        }
        self.root.as_ref().map(|x| x.right.in_parse(res));
    }

    fn pre_parse(&self, res: &mut Vec<String>) {
        if let Some(node) = self.root.as_ref() {
            res.push(node.key.to_string());
            node.left.pre_parse(res);
            node.right.pre_parse(res);
        }
    }
    pub fn print(&self) {
        let mut res1 = Vec::new();
        let mut res2 = Vec::new();
        self.in_parse(&mut res1);
        self.pre_parse(&mut res2);
        println!(" {}", res1.join(" "));
        println!(" {}", res2.join(" "));
    }
}


/* ========== Scanner ========== */

struct Scanner<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }

    fn read<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }

    fn char(&mut self) -> char {
        self.next::<String>().chars().next().unwrap()
    }
}

